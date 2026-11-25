mod diary;
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
    if trip.title.trim().is_empty() || trip.starts_at.trim().is_empty() {
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
            diary::trip_details,
            diary::add_rod,
            diary::add_interval,
            diary::add_catch,
            diary::set_outcome,
            weather::fetch_forecast,
            weather::cached_forecast
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
        });
    }
}
