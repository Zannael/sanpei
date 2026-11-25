use crate::AppError;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, sqlx::FromRow)]
pub struct Rod {
    pub id: i64,
    pub name: String,
}
#[derive(Serialize, sqlx::FromRow)]
pub struct Interval {
    pub id: i64,
    pub rod_id: i64,
    pub starts_at: String,
    pub ends_at: String,
    pub rig: String,
    pub bait: String,
}
#[derive(Serialize, sqlx::FromRow)]
pub struct Catch {
    pub id: i64,
    pub occurred_at: String,
    pub species: String,
    pub uncertain: bool,
    pub length_cm: Option<f64>,
    pub weight_g: Option<f64>,
    pub weight_estimated: bool,
    pub disposition: String,
}
#[derive(Serialize, sqlx::FromRow)]
pub struct Observation {
    pub id: i64,
    pub occurred_at: String,
    pub kind: String,
    pub notes: String,
}
#[derive(Deserialize)]
pub struct NewObservation {
    pub trip_id: i64,
    pub occurred_at: String,
    pub kind: String,
    pub notes: String,
}
#[derive(Serialize)]
pub struct Details {
    pub rods: Vec<Rod>,
    pub intervals: Vec<Interval>,
    pub catches: Vec<Catch>,
    pub observations: Vec<Observation>,
}
#[derive(Deserialize)]
pub struct NewInterval {
    pub rod_id: i64,
    pub starts_at: String,
    pub ends_at: String,
    pub rig: String,
    pub bait: String,
}
#[derive(Deserialize)]
pub struct NewCatch {
    pub trip_id: i64,
    pub occurred_at: String,
    pub species: String,
    pub uncertain: bool,
    pub length_cm: Option<f64>,
    pub weight_g: Option<f64>,
    pub weight_estimated: bool,
    pub disposition: String,
}
fn invalid(message: &str) -> AppError {
    AppError {
        code: "invalid_input",
        message: message.into(),
    }
}
#[tauri::command]
pub async fn trip_details(
    pool: tauri::State<'_, SqlitePool>,
    trip_id: i64,
) -> Result<Details, AppError> {
    let rods = sqlx::query_as::<_, Rod>("SELECT id,name FROM rods WHERE trip_id=? ORDER BY id")
        .bind(trip_id)
        .fetch_all(pool.inner())
        .await
        .map_err(AppError::database)?;
    let intervals=sqlx::query_as::<_,Interval>("SELECT i.id,i.rod_id,i.starts_at,i.ends_at,i.rig,i.bait FROM intervals i JOIN rods r ON r.id=i.rod_id WHERE r.trip_id=? ORDER BY i.starts_at").bind(trip_id).fetch_all(pool.inner()).await.map_err(AppError::database)?;
    let catches=sqlx::query_as::<_,Catch>("SELECT id,occurred_at,species,uncertain,length_cm,weight_g,weight_estimated,disposition FROM catches WHERE trip_id=? ORDER BY occurred_at").bind(trip_id).fetch_all(pool.inner()).await.map_err(AppError::database)?;
    let observations = sqlx::query_as::<_, Observation>(
        "SELECT id,occurred_at,kind,notes FROM observations WHERE trip_id=? ORDER BY occurred_at",
    )
    .bind(trip_id)
    .fetch_all(pool.inner())
    .await
    .map_err(AppError::database)?;
    Ok(Details {
        rods,
        intervals,
        catches,
        observations,
    })
}
#[tauri::command]
pub async fn add_rod(
    pool: tauri::State<'_, SqlitePool>,
    trip_id: i64,
    name: String,
) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(invalid("Inserisci il nome della canna."));
    }
    sqlx::query("INSERT INTO rods (trip_id,name) VALUES (?,?)")
        .bind(trip_id)
        .bind(name.trim())
        .execute(pool.inner())
        .await
        .map_err(AppError::database)?;
    Ok(())
}
#[tauri::command]
pub async fn add_interval(
    pool: tauri::State<'_, SqlitePool>,
    interval: NewInterval,
) -> Result<(), AppError> {
    if interval.starts_at.len() != 20
        || interval.ends_at.len() != 20
        || !interval.starts_at.ends_with('Z')
        || !interval.ends_at.ends_with('Z')
        || interval.starts_at >= interval.ends_at
    {
        return Err(invalid("Controlla inizio e fine dell'intervallo."));
    }
    sqlx::query("INSERT INTO intervals (rod_id,starts_at,ends_at,rig,bait) VALUES (?,?,?,?,?)")
        .bind(interval.rod_id)
        .bind(interval.starts_at)
        .bind(interval.ends_at)
        .bind(interval.rig)
        .bind(interval.bait)
        .execute(pool.inner())
        .await
        .map_err(|e| {
            if e.to_string().contains("interval_overlap") {
                invalid("Gli intervalli della stessa canna non possono sovrapporsi.")
            } else {
                AppError::database(e)
            }
        })?;
    Ok(())
}
#[tauri::command]
pub async fn add_catch(
    pool: tauri::State<'_, SqlitePool>,
    catch: NewCatch,
) -> Result<(), AppError> {
    if catch.species.trim().is_empty()
        || catch.occurred_at.is_empty()
        || !["released", "kept", "unknown"].contains(&catch.disposition.as_str())
        || catch.length_cm.is_some_and(|x| !x.is_finite() || x <= 0.0)
        || catch.weight_g.is_some_and(|x| !x.is_finite() || x <= 0.0)
    {
        return Err(invalid("Controlla specie, ora e misure della cattura."));
    }
    let mut tx = pool.begin().await.map_err(AppError::database)?;
    sqlx::query("INSERT INTO catches (trip_id,occurred_at,species,uncertain,length_cm,weight_g,weight_estimated,disposition) VALUES (?,?,?,?,?,?,?,?)").bind(catch.trip_id).bind(catch.occurred_at).bind(catch.species.trim()).bind(catch.uncertain).bind(catch.length_cm).bind(catch.weight_g).bind(catch.weight_estimated).bind(catch.disposition).execute(&mut *tx).await.map_err(AppError::database)?;
    sqlx::query("UPDATE trips SET outcome='catches' WHERE id=?")
        .bind(catch.trip_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::database)?;
    tx.commit().await.map_err(AppError::database)?;
    Ok(())
}
#[tauri::command]
pub async fn set_outcome(
    pool: tauri::State<'_, SqlitePool>,
    trip_id: i64,
    outcome: String,
) -> Result<(), AppError> {
    if !["incomplete", "zero", "catches"].contains(&outcome.as_str()) {
        return Err(invalid("Esito non valido."));
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM catches WHERE trip_id=?")
        .bind(trip_id)
        .fetch_one(pool.inner())
        .await
        .map_err(AppError::database)?;
    if (outcome == "zero" && count > 0) || (outcome == "catches" && count == 0) {
        return Err(invalid("L'esito non corrisponde alle catture registrate."));
    }
    sqlx::query("UPDATE trips SET outcome=? WHERE id=?")
        .bind(outcome)
        .bind(trip_id)
        .execute(pool.inner())
        .await
        .map_err(AppError::database)?;
    Ok(())
}

#[tauri::command]
pub async fn add_observation(
    pool: tauri::State<'_, SqlitePool>,
    observation: NewObservation,
) -> Result<(), AppError> {
    if observation.kind.trim().is_empty() || observation.occurred_at.trim().is_empty() {
        return Err(invalid("Inserisci tipo e ora dell'osservazione."));
    }
    sqlx::query("INSERT INTO observations (trip_id,occurred_at,kind,notes) VALUES (?,?,?,?)")
        .bind(observation.trip_id)
        .bind(observation.occurred_at)
        .bind(observation.kind.trim())
        .bind(observation.notes.trim())
        .execute(pool.inner())
        .await
        .map_err(AppError::database)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_diary_item(
    pool: tauri::State<'_, SqlitePool>,
    trip_id: i64,
    item_id: i64,
    kind: String,
) -> Result<(), AppError> {
    let query = match kind.as_str() {
        "rod" => "DELETE FROM rods WHERE id=? AND trip_id=?",
        "interval" => {
            "DELETE FROM intervals WHERE id=? AND rod_id IN (SELECT id FROM rods WHERE trip_id=?)"
        }
        "catch" => "DELETE FROM catches WHERE id=? AND trip_id=?",
        "observation" => "DELETE FROM observations WHERE id=? AND trip_id=?",
        _ => return Err(invalid("Elemento non valido.")),
    };
    let mut tx = pool.begin().await.map_err(AppError::database)?;
    let result = sqlx::query(query)
        .bind(item_id)
        .bind(trip_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::database)?;
    if result.rows_affected() != 1 {
        return Err(invalid("Elemento non trovato."));
    }
    if kind == "catch" {
        sqlx::query("UPDATE trips SET outcome='incomplete' WHERE id=? AND outcome='catches' AND NOT EXISTS (SELECT 1 FROM catches WHERE trip_id=?)").bind(trip_id).bind(trip_id).execute(&mut *tx).await.map_err(AppError::database)?;
    }
    tx.commit().await.map_err(AppError::database)?;
    Ok(())
}
