mod backup;
mod diary;
mod planner;
mod time;
mod weather;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::str::FromStr;
use tauri::Manager;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Trip {
    id: i64,
    title: String,
    starts_at: String,
    location: String,
    notes: String,
    outcome: String,
}
#[derive(Debug, Deserialize)]
struct NewTrip {
    title: String,
    starts_at: String,
    location: String,
    notes: String,
}
#[derive(Debug, Serialize)]
struct AppError {
    code: &'static str,
    message: String,
}
impl AppError {
    fn database(_: sqlx::Error) -> Self {
        Self {
            code: "database_error",
            message: "Impossibile completare l'operazione sul diario.".into(),
        }
    }
}
#[tauri::command]
async fn list_trips(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<Trip>, AppError> {
    sqlx::query_as::<_, Trip>("SELECT id,title,starts_at,location,notes,outcome FROM trips ORDER BY starts_at DESC, id DESC").fetch_all(pool.inner()).await.map_err(AppError::database)
}
#[tauri::command]
async fn create_trip(pool: tauri::State<'_, SqlitePool>, trip: NewTrip) -> Result<(), AppError> {
    if trip.title.trim().is_empty() || !time::valid_utc(&trip.starts_at, false) {
        return Err(AppError {
            code: "invalid_trip",
            message: "Inserisci un titolo e una data.".into(),
        });
    }
    sqlx::query("INSERT INTO trips (title,starts_at,location,notes) VALUES (?,?,?,?)")
        .bind(trip.title.trim())
        .bind(trip.starts_at)
        .bind(trip.location.trim())
        .bind(trip.notes.trim())
        .execute(pool.inner())
        .await
        .map_err(AppError::database)?;
    Ok(())
}
#[tauri::command]
async fn update_trip(
    pool: tauri::State<'_, SqlitePool>,
    trip_id: i64,
    trip: NewTrip,
) -> Result<(), AppError> {
    if trip.title.trim().is_empty() || !time::valid_utc(&trip.starts_at, false) {
        return Err(AppError {
            code: "invalid_trip",
            message: "Inserisci un titolo e una data.".into(),
        });
    }
    let result = sqlx::query("UPDATE trips SET title=?,starts_at=?,location=?,notes=? WHERE id=?")
        .bind(trip.title.trim())
        .bind(trip.starts_at)
        .bind(trip.location.trim())
        .bind(trip.notes.trim())
        .bind(trip_id)
        .execute(pool.inner())
        .await
        .map_err(AppError::database)?;
    if result.rows_affected() != 1 {
        return Err(AppError {
            code: "not_found",
            message: "Uscita non trovata.".into(),
        });
    }
    Ok(())
}
#[tauri::command]
async fn delete_trip(pool: tauri::State<'_, SqlitePool>, trip_id: i64) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM trips WHERE id=?")
        .bind(trip_id)
        .execute(pool.inner())
        .await
        .map_err(AppError::database)?;
    if result.rows_affected() != 1 {
        return Err(AppError {
            code: "not_found",
            message: "Uscita non trovata.".into(),
        });
    }
    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let db = dir.join("sanpei.sqlite3");
            let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db.display()))?
                .create_if_missing(true)
                .foreign_keys(true);
            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect_with(options)
                    .await?;
                sqlx::migrate!("./migrations").run(&pool).await?;
                Ok::<_, Box<dyn std::error::Error>>(pool)
            })?;
            app.manage(pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_trips,
            create_trip,
            update_trip,
            delete_trip,
            diary::trip_details,
            diary::add_rod,
            diary::add_interval,
            diary::add_catch,
            diary::add_observation,
            diary::delete_diary_item,
            diary::set_outcome,
            weather::fetch_forecast,
            weather::cached_forecast,
            backup::backup_database,
            backup::restore_database,
            planner::evaluate_scenario,
            planner::save_plan,
            planner::list_plans
        ])
        .run(tauri::generate_context!())
        .expect("Errore avvio Sanpei");
}

#[cfg(test)]
mod tests {
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    async fn database() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[test]
    fn intervals_allow_two_rods_but_reject_overlap_on_same_rod() {
        tauri::async_runtime::block_on(async {
            let pool = database().await;
            sqlx::query(
                "INSERT INTO trips (title, starts_at) VALUES ('test', '2026-01-01T00:00:00Z')",
            )
            .execute(&pool)
            .await
            .unwrap();
            sqlx::query("INSERT INTO rods (trip_id, name) VALUES (1, 'A'), (1, 'B')")
                .execute(&pool)
                .await
                .unwrap();
            let add = |rod: i64, start: &'static str, end: &'static str| {
                let pool = pool.clone();
                async move {
                    sqlx::query(
                        "INSERT INTO intervals (rod_id, starts_at, ends_at) VALUES (?, ?, ?)",
                    )
                    .bind(rod)
                    .bind(start)
                    .bind(end)
                    .execute(&pool)
                    .await
                }
            };
            assert!(add(1, "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z")
                .await
                .is_ok());
            assert!(add(2, "2026-01-01T10:30:00Z", "2026-01-01T11:30:00Z")
                .await
                .is_ok());
            assert!(add(1, "2026-01-01T10:30:00Z", "2026-01-01T11:30:00Z")
                .await
                .is_err());
            assert!(add(1, "2026-01-01T11:00:00Z", "2026-01-01T12:00:00Z")
                .await
                .is_ok());
            let invalid=sqlx::query("INSERT INTO intervals(rod_id,starts_at,ends_at,pause_minutes,ineffective_minutes) VALUES (2,'2026-01-01T12:00:00Z','2026-01-01T13:00:00Z',50,20)").execute(&pool).await;
            assert!(invalid.is_err());
            let valid=sqlx::query("INSERT INTO intervals(rod_id,starts_at,ends_at,pause_minutes,ineffective_minutes) VALUES (2,'2026-01-01T12:00:00Z','2026-01-01T13:00:00Z',30,20)").execute(&pool).await;
            assert!(valid.is_ok());
        });
    }
    #[test]
    fn deleting_trip_removes_dependent_records() {
        tauri::async_runtime::block_on(async {
            let pool = database().await;
            sqlx::query(
                "INSERT INTO trips(title,starts_at) VALUES ('synthetic','2026-01-01T00:00:00Z')",
            )
            .execute(&pool)
            .await
            .unwrap();
            sqlx::query("INSERT INTO rods(trip_id,name) VALUES (1,'test')")
                .execute(&pool)
                .await
                .unwrap();
            sqlx::query("INSERT INTO intervals(rod_id,starts_at,ends_at) VALUES (1,'2026-01-01T01:00:00Z','2026-01-01T02:00:00Z')").execute(&pool).await.unwrap();
            sqlx::query("INSERT INTO observations(trip_id,occurred_at,kind) VALUES (1,'2026-01-01T01:00:00Z','mare')").execute(&pool).await.unwrap();
            sqlx::query("INSERT INTO catches(trip_id,occurred_at,species,disposition) VALUES (1,'2026-01-01T01:00:00Z','test','released')").execute(&pool).await.unwrap();
            sqlx::query("DELETE FROM trips WHERE id=1")
                .execute(&pool)
                .await
                .unwrap();
            for table in ["rods", "intervals", "observations", "catches"] {
                let count: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {table}"))
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                assert_eq!(count, 0);
            }
        });
    }
}
