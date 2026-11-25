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
