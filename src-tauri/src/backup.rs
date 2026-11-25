use crate::AppError;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

fn error(code: &'static str, message: &str) -> AppError {
    AppError {
        code,
        message: message.into(),
    }
}

// A private temporary directory on the destination filesystem lets hard_link publish
// a fully verified snapshot atomically without replacing any existing file.
async fn snapshot(pool: &SqlitePool, destination: &Path) -> Result<(), AppError> {
    let parent = destination
        .parent()
        .filter(|p| p.is_dir())
        .ok_or_else(|| error("invalid_path", "Cartella di destinazione non disponibile."))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| error("clock_error", "Orologio di sistema non disponibile."))?
        .as_nanos();
    let directory = parent.join(format!(".sanpei-backup-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&directory)
        .map_err(|_| error("backup_failed", "Impossibile creare la copia temporanea."))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).is_err() {
            let _ = std::fs::remove_dir(&directory);
            return Err(error(
                "backup_failed",
                "Impossibile proteggere la copia temporanea.",
            ));
        }
    }
    let temporary = directory.join("snapshot.sqlite3");
    let result = async {
        let name = temporary
            .to_str()
            .ok_or_else(|| error("invalid_path", "Percorso non supportato."))?;
        sqlx::query("VACUUM INTO ?")
            .bind(name)
            .execute(pool)
            .await
            .map_err(AppError::database)?;
        let verify = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(&temporary)
                .read_only(true),
        )
        .await
        .map_err(AppError::database)?;
        let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&verify)
            .await
            .map_err(AppError::database)?;
        verify.close().await;
        if integrity != "ok" {
            return Err(error(
                "invalid_backup",
                "La copia non ha superato il controllo di integrità.",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&temporary, std::fs::Permissions::from_mode(0o600))
                .map_err(|_| error("backup_failed", "Impossibile proteggere la copia."))?;
        }
        std::fs::hard_link(&temporary, destination).map_err(|_| {
            error(
                "backup_file_error",
                "Impossibile salvare: scegli un file che non esiste già.",
            )
        })?;
        Ok(())
    }
    .await;
    let _ = std::fs::remove_file(&temporary);
    let _ = std::fs::remove_dir(&directory);
    result
}

#[tauri::command]
pub async fn backup_database(pool: tauri::State<'_, SqlitePool>) -> Result<bool, AppError> {
    let selected = rfd::AsyncFileDialog::new()
        .set_title("Salva una copia del diario")
        .set_file_name("sanpei-backup.sqlite3")
        .add_filter("Database SQLite", &["sqlite3"])
        .save_file()
        .await;
    let Some(file) = selected else {
        return Ok(false);
    };
    snapshot(pool.inner(), file.path()).await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snapshot_is_readable_and_cannot_overwrite() {
        tauri::async_runtime::block_on(async {
            let dir = std::env::temp_dir().join(format!(
                "sanpei-backup-test-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir(&dir).unwrap();
            let input = dir.join("input.sqlite3");
            let output = dir.join("output.sqlite3");
            let db = SqlitePool::connect_with(
                SqliteConnectOptions::new()
                    .filename(&input)
                    .create_if_missing(true),
            )
            .await
            .unwrap();
            sqlx::query("CREATE TABLE example (value TEXT NOT NULL)")
                .execute(&db)
                .await
                .unwrap();
            sqlx::query("INSERT INTO example VALUES ('synthetic')")
                .execute(&db)
                .await
                .unwrap();
            snapshot(&db, &output).await.unwrap();
            let copy = SqlitePool::connect_with(
                SqliteConnectOptions::new()
                    .filename(&output)
                    .read_only(true),
            )
            .await
            .unwrap();
            let value: String = sqlx::query_scalar("SELECT value FROM example")
                .fetch_one(&copy)
                .await
                .unwrap();
            assert_eq!(value, "synthetic");
            copy.close().await;
            assert!(snapshot(&db, &output).await.is_err());
            let copy = SqlitePool::connect_with(
                SqliteConnectOptions::new()
                    .filename(&output)
                    .read_only(true),
            )
            .await
            .unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, String>("SELECT value FROM example")
                    .fetch_one(&copy)
                    .await
                    .unwrap(),
                "synthetic"
            );
            copy.close().await;
            db.close().await;
            std::fs::remove_dir_all(dir).unwrap();
        });
    }
}

// Only the schema explicitly handled below is accepted. A later migration must
// extend the copy list and its tests before restore is enabled for that schema.
const RESTORE_SCHEMA_VERSION: i64 = 3;

async fn restore(pool: &SqlitePool, source: &Path, recovery_dir: &Path) -> Result<(), AppError> {
    let source = std::fs::canonicalize(source)
        .map_err(|_| error("invalid_backup", "File non disponibile."))?;
    let mut url = reqwest::Url::from_file_path(&source)
        .map_err(|_| error("invalid_path", "Percorso non supportato."))?;
    url.set_query(Some("mode=ro"));
    let mut conn = pool.acquire().await.map_err(AppError::database)?;
    let main_path: String =
        sqlx::query_scalar("SELECT file FROM pragma_database_list WHERE name='main'")
            .fetch_one(&mut *conn)
            .await
            .map_err(AppError::database)?;
    if std::fs::canonicalize(main_path).ok().as_ref() == Some(&source) {
        return Err(error(
            "invalid_backup",
            "Scegli una copia, non il database attualmente in uso.",
        ));
    }
    sqlx::query("ATTACH DATABASE ? AS imported")
        .bind(url.as_str())
        .execute(&mut *conn)
        .await
        .map_err(|_| error("invalid_backup", "Impossibile aprire il backup."))?;
    let result = async {
        let integrity: String = sqlx::query_scalar("PRAGMA imported.integrity_check").fetch_one(&mut *conn).await.map_err(AppError::database)?;
        if integrity != "ok" { return Err(error("invalid_backup", "Il backup non ha superato il controllo di integrità.")); }
        if sqlx::query("PRAGMA imported.foreign_key_check").fetch_optional(&mut *conn).await.map_err(AppError::database)?.is_some() {
            return Err(error("invalid_backup", "Il backup contiene riferimenti non validi."));
        }
        let current: Vec<(i64, Vec<u8>, bool)> = sqlx::query_as("SELECT version,checksum,success FROM main._sqlx_migrations ORDER BY version").fetch_all(&mut *conn).await.map_err(AppError::database)?;
        let incoming: Vec<(i64, Vec<u8>, bool)> = sqlx::query_as("SELECT version,checksum,success FROM imported._sqlx_migrations ORDER BY version").fetch_all(&mut *conn).await.map_err(|_| error("invalid_backup", "Il file non è un backup Sanpei compatibile."))?;
        if current != incoming || current.last().map(|row|row.0) != Some(RESTORE_SCHEMA_VERSION) || current.iter().any(|row| !row.2) {
            return Err(error("incompatible_backup", "La versione del backup non è compatibile con questa app."));
        }
        // Serialize writes before taking the pre-restore recovery snapshot.
        sqlx::query("BEGIN IMMEDIATE").execute(&mut *conn).await.map_err(AppError::database)?;
        let operation = async {
            std::fs::create_dir_all(recovery_dir).map_err(|_| error("backup_failed", "Impossibile conservare i dati correnti."))?;
            let nonce=SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| error("clock_error", "Orologio non disponibile."))?.as_nanos();
            let recovery = recovery_dir.join(format!("before-restore-{nonce}.sqlite3"));
            snapshot(pool, &recovery).await?;
            for statement in [
                "DELETE FROM intervals", "DELETE FROM catches", "DELETE FROM observations", "DELETE FROM rods", "DELETE FROM trips", "DELETE FROM forecast_cache",
                "INSERT INTO trips(id,title,starts_at,location,notes,outcome,created_at) SELECT id,title,starts_at,location,notes,outcome,created_at FROM imported.trips",
                "INSERT INTO rods(id,trip_id,name) SELECT id,trip_id,name FROM imported.rods",
                "INSERT INTO intervals(id,rod_id,starts_at,ends_at,rig,bait) SELECT id,rod_id,starts_at,ends_at,rig,bait FROM imported.intervals ORDER BY id",
                "INSERT INTO catches(id,trip_id,occurred_at,species,uncertain,length_cm,weight_g,weight_estimated,disposition) SELECT id,trip_id,occurred_at,species,uncertain,length_cm,weight_g,weight_estimated,disposition FROM imported.catches",
                "INSERT INTO observations(id,trip_id,occurred_at,kind,notes) SELECT id,trip_id,occurred_at,kind,notes FROM imported.observations",
                "INSERT INTO forecast_cache(latitude,longitude,weather,marine,retrieved_at) SELECT latitude,longitude,weather,marine,retrieved_at FROM imported.forecast_cache",
            ] { sqlx::query(statement).execute(&mut *conn).await.map_err(AppError::database)?; }
            sqlx::query("COMMIT").execute(&mut *conn).await.map_err(AppError::database)?;
            Ok(())
        }.await;
        if operation.is_err() { let _=sqlx::query("ROLLBACK").execute(&mut *conn).await; }
        operation
    }.await;
    let _ = sqlx::query("DETACH DATABASE imported")
        .execute(&mut *conn)
        .await;
    result
}

#[tauri::command]
pub async fn restore_database(
    pool: tauri::State<'_, SqlitePool>,
    app: tauri::AppHandle,
) -> Result<bool, AppError> {
    use tauri::Manager;
    let Some(file) = rfd::AsyncFileDialog::new()
        .set_title("Scegli una copia Sanpei")
        .add_filter("Database SQLite", &["sqlite3"])
        .pick_file()
        .await
    else {
        return Ok(false);
    };
    let confirmation = rfd::AsyncMessageDialog::new().set_title("Ripristinare il diario?").set_description("I dati correnti saranno sostituiti. Sanpei conserverà prima una copia locale di recupero.").set_buttons(rfd::MessageButtons::YesNo).show().await;
    if confirmation != rfd::MessageDialogResult::Yes {
        return Ok(false);
    }
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|_| error("invalid_path", "Cartella dati non disponibile."))?
        .join("recovery");
    restore(pool.inner(), file.path(), &directory).await?;
    Ok(true)
}

#[cfg(test)]
mod restore_tests {
    use super::*;
    #[test]
    fn restore_recovers_data_and_rejects_incompatible_backups() {
        tauri::async_runtime::block_on(async {
            let dir = std::env::temp_dir().join(format!(
                "sanpei-restore-test-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir(&dir).unwrap();
            let db = SqlitePool::connect_with(
                SqliteConnectOptions::new()
                    .filename(dir.join("live.sqlite3"))
                    .create_if_missing(true)
                    .foreign_keys(true),
            )
            .await
            .unwrap();
            sqlx::migrate!("./migrations").run(&db).await.unwrap();
            sqlx::query(
                "INSERT INTO trips(title,starts_at) VALUES ('original','2026-01-01T00:00:00Z')",
            )
            .execute(&db)
            .await
            .unwrap();
            let original = dir.join("backup.sqlite3");
            snapshot(&db, &original).await.unwrap();
            sqlx::query("UPDATE trips SET title='changed'")
                .execute(&db)
                .await
                .unwrap();
            restore(&db, &original, &dir.join("recovery"))
                .await
                .unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, String>("SELECT title FROM trips")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                "original"
            );
            let invalid = dir.join("invalid.sqlite3");
            std::fs::write(&invalid, b"not sqlite").unwrap();
            assert!(restore(&db, &invalid, &dir.join("recovery")).await.is_err());
            assert!(
                restore(&db, &dir.join("live.sqlite3"), &dir.join("recovery"))
                    .await
                    .is_err()
            );
            let altered = SqlitePool::connect_with(SqliteConnectOptions::new().filename(&original))
                .await
                .unwrap();
            sqlx::query("UPDATE _sqlx_migrations SET checksum=X'00' WHERE version=1")
                .execute(&altered)
                .await
                .unwrap();
            altered.close().await;
            assert!(restore(&db, &original, &dir.join("recovery"))
                .await
                .is_err());
            assert_eq!(
                sqlx::query_scalar::<_, String>("SELECT title FROM trips")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                "original"
            );
            let count = std::fs::read_dir(dir.join("recovery")).unwrap().count();
            assert_eq!(count, 1);
            db.close().await;
            std::fs::remove_dir_all(dir).unwrap();
        });
    }
}
