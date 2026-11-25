use crate::AppError;
use serde::Serialize;
use sqlx::SqlitePool;
use std::time::Duration;
#[derive(Serialize)]
pub struct Forecast {
    pub weather: Option<serde_json::Value>,
    pub marine: Option<serde_json::Value>,
    pub retrieved_at: String,
}
#[tauri::command]
pub async fn fetch_forecast(
    latitude: f64,
    longitude: f64,
    pool: tauri::State<'_, SqlitePool>,
) -> Result<Forecast, AppError> {
    if !latitude.is_finite()
        || !longitude.is_finite()
        || !(-90.0..=90.0).contains(&latitude)
        || !(-180.0..=180.0).contains(&longitude)
    {
        return Err(AppError {
            code: "invalid_coordinates",
            message: "Coordinate non valide.".into(),
        });
    }
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|_| AppError {
            code: "network_error",
            message: "Impossibile preparare la richiesta.".into(),
        })?;
    let lat = latitude.to_string();
    let lon = longitude.to_string();
    let query = [
        ("latitude", lat.as_str()),
        ("longitude", lon.as_str()),
        ("timezone", "Europe/Rome"),
    ];
    let weather = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&query)
        .query(&[(
            "hourly",
            "temperature_2m,precipitation,wind_speed_10m,wind_direction_10m,wind_gusts_10m",
        )])
        .send()
        .await
        .ok();
    let marine = client
        .get("https://marine-api.open-meteo.com/v1/marine")
        .query(&query)
        .query(&[(
            "hourly",
            "wave_height,wave_direction,wave_period,sea_surface_temperature",
        )])
        .send()
        .await
        .ok();
    let weather = match weather {
        Some(r) if r.status().is_success() => r.json().await.ok(),
        _ => None,
    };
    let marine = match marine {
        Some(r) if r.status().is_success() => r.json().await.ok(),
        _ => None,
    };
    if weather.is_none() && marine.is_none() {
        return Err(AppError {
            code: "forecast_unavailable",
            message: "Previsioni non disponibili. Riprova quando sei connesso.".into(),
        });
    }
    let retrieved_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| AppError {
            code: "clock_error",
            message: "Orologio di sistema non disponibile.".into(),
        })?
        .as_secs()
        .to_string();
    sqlx::query("INSERT INTO forecast_cache (latitude,longitude,weather,marine,retrieved_at) VALUES (?,?,?,?,?) ON CONFLICT(latitude,longitude) DO UPDATE SET weather=excluded.weather, marine=excluded.marine,retrieved_at=excluded.retrieved_at")
        .bind(latitude).bind(longitude)
        .bind(weather.as_ref().map(serde_json::Value::to_string))
        .bind(marine.as_ref().map(serde_json::Value::to_string))
        .bind(&retrieved_at).execute(pool.inner()).await.map_err(AppError::database)?;
    Ok(Forecast {
        weather,
        marine,
        retrieved_at,
    })
}

#[tauri::command]
pub async fn cached_forecast(
    latitude: f64,
    longitude: f64,
    pool: tauri::State<'_, SqlitePool>,
) -> Result<Option<Forecast>, AppError> {
    use sqlx::Row;
    let row = sqlx::query(
        "SELECT weather,marine,retrieved_at FROM forecast_cache WHERE latitude=? AND longitude=?",
    )
    .bind(latitude)
    .bind(longitude)
    .fetch_optional(pool.inner())
    .await
    .map_err(AppError::database)?;
    row.map(|row| {
        let weather: Option<String> = row.try_get("weather").map_err(AppError::database)?;
        let marine: Option<String> = row.try_get("marine").map_err(AppError::database)?;
        Ok(Forecast {
            weather: weather
                .map(|x| serde_json::from_str(&x))
                .transpose()
                .map_err(|_| AppError {
                    code: "invalid_cache",
                    message: "Dati in cache non validi.".into(),
                })?,
            marine: marine
                .map(|x| serde_json::from_str(&x))
                .transpose()
                .map_err(|_| AppError {
                    code: "invalid_cache",
                    message: "Dati in cache non validi.".into(),
                })?,
            retrieved_at: row.try_get("retrieved_at").map_err(AppError::database)?,
        })
    })
    .transpose()
}
