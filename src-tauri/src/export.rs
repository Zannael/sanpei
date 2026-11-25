use crate::AppError;
use serde_json::{json, Value};
use sqlx::{Row, SqlitePool};
use std::{
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
fn error(code: &'static str, message: &str) -> AppError {
    AppError {
        code,
        message: message.into(),
    }
}

async fn json_array(conn: &mut sqlx::SqliteConnection, query: &str) -> Result<Value, AppError> {
    let raw: String = sqlx::query_scalar(query)
        .fetch_one(conn)
        .await
        .map_err(AppError::database)?;
    serde_json::from_str(&raw)
        .map_err(|_| error("export_failed", "Impossibile preparare l'esportazione."))
}
async fn build(pool: &SqlitePool, kind: &str) -> Result<Vec<u8>, AppError> {
    let mut tx = pool.begin().await.map_err(AppError::database)?;
    let output = match kind {
        "json" => {
            let mut data = serde_json::Map::new();
            for (key,sql) in [
    ("trips","SELECT json_group_array(json_object('id',id,'title',title,'starts_at',starts_at,'location',location,'notes',notes,'outcome',outcome,'created_at',created_at)) FROM (SELECT * FROM trips ORDER BY id)"),
    ("rods","SELECT json_group_array(json_object('id',id,'trip_id',trip_id,'name',name)) FROM (SELECT * FROM rods ORDER BY id)"),
    ("intervals","SELECT json_group_array(json_object('id',id,'rod_id',rod_id,'starts_at',starts_at,'ends_at',ends_at,'rig',rig,'bait',bait,'pause_minutes',pause_minutes,'ineffective_minutes',ineffective_minutes)) FROM (SELECT * FROM intervals ORDER BY id)"),
    ("catches","SELECT json_group_array(json_object('id',id,'trip_id',trip_id,'occurred_at',occurred_at,'species',species,'uncertain',uncertain,'length_cm',length_cm,'weight_g',weight_g,'weight_estimated',weight_estimated,'disposition',disposition)) FROM (SELECT * FROM catches ORDER BY id)"),
    ("observations","SELECT json_group_array(json_object('id',id,'trip_id',trip_id,'occurred_at',occurred_at,'kind',kind,'notes',notes)) FROM (SELECT * FROM observations ORDER BY id)"),
    ("positions","SELECT json_group_array(json_object('id',id,'trip_id',trip_id,'role',role,'latitude',latitude,'longitude',longitude,'origin',origin,'accuracy_m',accuracy_m)) FROM (SELECT * FROM trip_positions ORDER BY id)"),
    ("plans","SELECT json_group_array(json_object('id',id,'created_at',created_at,'scenario',json(scenario_json),'result',json(result_json),'content',json(rules_json),'engine_version',engine_version,'content_version',content_version)) FROM (SELECT * FROM plans ORDER BY id)"),
    ("trip_plans","SELECT json_group_array(json_object('trip_id',trip_id,'plan_id',plan_id,'followed',followed,'notes',notes)) FROM (SELECT * FROM trip_plans ORDER BY trip_id)"),
    ("research_packages","SELECT json_group_array(json_object('package_id',package_id,'content_version',content_version,'region',region,'bundle',json(bundle_json),'active',active,'imported_at',imported_at)) FROM (SELECT * FROM research_packages ORDER BY package_id,content_version)"),
    ("forecast_cache","SELECT json_group_array(json_object('latitude',latitude,'longitude',longitude,'weather',json(weather),'marine',json(marine),'retrieved_at',retrieved_at)) FROM (SELECT * FROM forecast_cache ORDER BY latitude,longitude)"),
   ] {data.insert(key.into(),json_array(&mut tx,sql).await?);}
            let active: Option<String> = sqlx::query_scalar(
                "SELECT bundle_json FROM research_packages WHERE region='Lazio' AND active=1",
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::database)?;
            let active: Value = match active {
                Some(raw) => serde_json::from_str(&raw)
                    .map_err(|_| error("export_failed", "Pacchetto attivo non leggibile."))?,
                None => serde_json::to_value(crate::content::builtin()?)
                    .map_err(|_| error("export_failed", "Pacchetto iniziale non leggibile."))?,
            };
            data.insert("active_research".into(), active);
            serde_json::to_vec_pretty(
                &json!({"format":"sanpei-export","schema_version":1,"data":data}),
            )
            .map_err(|_| error("export_failed", "Impossibile preparare il JSON."))?
        }
        "geojson" => {
            let points=json_array(&mut tx,"SELECT json_group_array(json_object('type','Feature','geometry',json_object('type','Point','coordinates',json_array(longitude,latitude)),'properties',json_object('trip_id',trip_id,'role',role,'origin',origin,'accuracy_m',accuracy_m))) FROM (SELECT * FROM trip_positions ORDER BY id)").await?;
            serde_json::to_vec_pretty(&json!({"type":"FeatureCollection","features":points}))
                .map_err(|_| error("export_failed", "Impossibile preparare il GeoJSON."))?
        }
        "trips_csv" => {
            let rows = sqlx::query(
                "SELECT id,title,starts_at,location,notes,outcome FROM trips ORDER BY id",
            )
            .fetch_all(&mut *tx)
            .await
            .map_err(AppError::database)?;
            let mut csv = String::from("trip_id,title,start_utc,location,notes,outcome\r\n");
            for row in rows {
                let fields: Vec<String> = vec![
                    row.try_get::<i64, _>("id")
                        .map_err(AppError::database)?
                        .to_string(),
                    row.try_get("title").map_err(AppError::database)?,
                    row.try_get("starts_at").map_err(AppError::database)?,
                    row.try_get("location").map_err(AppError::database)?,
                    row.try_get("notes").map_err(AppError::database)?,
                    row.try_get("outcome").map_err(AppError::database)?,
                ];
                csv.push_str(
                    &fields
                        .iter()
                        .map(|x| csv_cell(x))
                        .collect::<Vec<_>>()
                        .join(","),
                );
                csv.push_str("\r\n");
            }
            csv.into_bytes()
        }
        "intervals_csv" => {
            let rows=sqlx::query("SELECT t.id AS trip_id,t.title,t.starts_at AS trip_start,t.outcome,r.name AS rod,i.starts_at,i.ends_at,i.rig,i.bait,i.pause_minutes,i.ineffective_minutes FROM intervals i JOIN rods r ON r.id=i.rod_id JOIN trips t ON t.id=r.trip_id ORDER BY t.id,r.id,i.starts_at").fetch_all(&mut *tx).await.map_err(AppError::database)?;
            let mut csv=String::from("trip_id,title,trip_start_utc,outcome,rod,start_utc,end_utc,rig,bait,pause_minutes,ineffective_minutes\r\n");
            for row in rows {
                let fields = vec![
                    row.try_get::<i64, _>("trip_id")
                        .map_err(AppError::database)?
                        .to_string(),
                    row.try_get("title").map_err(AppError::database)?,
                    row.try_get("trip_start").map_err(AppError::database)?,
                    row.try_get("outcome").map_err(AppError::database)?,
                    row.try_get("rod").map_err(AppError::database)?,
                    row.try_get("starts_at").map_err(AppError::database)?,
                    row.try_get("ends_at").map_err(AppError::database)?,
                    row.try_get("rig").map_err(AppError::database)?,
                    row.try_get("bait").map_err(AppError::database)?,
                    row.try_get::<Option<i64>, _>("pause_minutes")
                        .map_err(AppError::database)?
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                    row.try_get::<Option<i64>, _>("ineffective_minutes")
                        .map_err(AppError::database)?
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                ];
                csv.push_str(
                    &fields
                        .iter()
                        .map(|v| csv_cell(v))
                        .collect::<Vec<_>>()
                        .join(","),
                );
                csv.push_str("\r\n");
            }
            csv.into_bytes()
        }
        "catches_csv" => {
            let rows=sqlx::query("SELECT t.id AS trip_id,t.title,c.occurred_at,c.species,c.uncertain,c.length_cm,c.weight_g,c.weight_estimated,c.disposition FROM catches c JOIN trips t ON t.id=c.trip_id ORDER BY t.id,c.occurred_at").fetch_all(&mut *tx).await.map_err(AppError::database)?;
            let mut csv=String::from("trip_id,title,occurred_at_utc,species,uncertain,length_cm,weight_g,weight_estimated,disposition\r\n");
            for row in rows {
                let fields = vec![
                    row.try_get::<i64, _>("trip_id")
                        .map_err(AppError::database)?
                        .to_string(),
                    row.try_get("title").map_err(AppError::database)?,
                    row.try_get("occurred_at").map_err(AppError::database)?,
                    row.try_get("species").map_err(AppError::database)?,
                    row.try_get::<i64, _>("uncertain")
                        .map_err(AppError::database)?
                        .to_string(),
                    row.try_get::<Option<f64>, _>("length_cm")
                        .map_err(AppError::database)?
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                    row.try_get::<Option<f64>, _>("weight_g")
                        .map_err(AppError::database)?
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                    row.try_get::<i64, _>("weight_estimated")
                        .map_err(AppError::database)?
                        .to_string(),
                    row.try_get("disposition").map_err(AppError::database)?,
                ];
                csv.push_str(
                    &fields
                        .iter()
                        .map(|v| csv_cell(v))
                        .collect::<Vec<_>>()
                        .join(","),
                );
                csv.push_str("\r\n");
            }
            csv.into_bytes()
        }
        _ => return Err(error("invalid_export", "Formato non supportato.")),
    };
    tx.commit().await.map_err(AppError::database)?;
    Ok(output)
}
fn csv_cell(value: &str) -> String {
    let trimmed = value.trim_start_matches(|c: char| c.is_whitespace() || c.is_control());
    let guarded = if trimmed.starts_with(['=', '+', '-', '@']) {
        format!("'{value}")
    } else {
        value.to_string()
    };
    format!("\"{}\"", guarded.replace('"', "\"\""))
}
fn write_new(destination: &Path, data: &[u8]) -> Result<(), AppError> {
    let parent = destination
        .parent()
        .filter(|p| p.is_dir())
        .ok_or_else(|| error("invalid_path", "Cartella di destinazione non disponibile."))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| error("clock_error", "Orologio non disponibile."))?
        .as_nanos();
    let dir = parent.join(format!(".sanpei-export-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&dir)
        .map_err(|_| error("export_failed", "Impossibile creare il file temporaneo."))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).is_err() {
            let _ = std::fs::remove_dir(&dir);
            return Err(error(
                "export_failed",
                "Impossibile proteggere il file temporaneo.",
            ));
        }
    }
    let temp = dir.join("data");
    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&temp)
            .map_err(|_| error("export_failed", "Impossibile creare il file temporaneo."))?;
        file.write_all(data)
            .and_then(|_| file.sync_all())
            .map_err(|_| error("export_failed", "Impossibile scrivere il file."))?;
        drop(file);
        std::fs::hard_link(&temp, destination).map_err(|e| {
            if e.kind()==std::io::ErrorKind::AlreadyExists { error("export_file_error", "Il file esiste già: scegli un altro nome.") }
            else { error("export_file_error", "Impossibile salvare nel percorso scelto; verifica permessi e supporto del file system.") }
        })?;
        Ok(())
    })();
    let _ = std::fs::remove_file(&temp);
    let _ = std::fs::remove_dir(&dir);
    result
}
#[tauri::command]
pub async fn export_data(
    pool: tauri::State<'_, SqlitePool>,
    kind: String,
) -> Result<bool, AppError> {
    let (filename, extension) = match kind.as_str() {
        "json" => ("sanpei-diario.json", "json"),
        "geojson" => ("sanpei-posizioni.geojson", "geojson"),
        "trips_csv" => ("sanpei-uscite.csv", "csv"),
        "intervals_csv" => ("sanpei-intervalli.csv", "csv"),
        "catches_csv" => ("sanpei-catture.csv", "csv"),
        _ => return Err(error("invalid_export", "Formato non supportato.")),
    };
    let Some(file) = rfd::AsyncFileDialog::new()
        .set_title("Esporta dati del diario")
        .set_file_name(filename)
        .add_filter("File di esportazione", &[extension])
        .save_file()
        .await
    else {
        return Ok(false);
    };
    let bytes = build(pool.inner(), &kind).await?;
    write_new(file.path(), &bytes)?;
    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn export_does_not_replace_existing_file() {
        let dir = std::env::temp_dir().join(format!(
            "sanpei-export-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("data.csv");
        std::fs::write(&path, b"existing").unwrap();
        assert!(write_new(&path, b"new").is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"existing");
        let new = dir.join("new.csv");
        write_new(&new, b"test").unwrap();
        assert_eq!(std::fs::read(&new).unwrap(), b"test");
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn exports_preserve_missing_values_and_guard_csv() {
        tauri::async_runtime::block_on(async {
            let db = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            sqlx::migrate!("./migrations").run(&db).await.unwrap();
            sqlx::query("INSERT INTO trips(title,starts_at,outcome) VALUES ('=test, uno','2026-01-01T00:00:00Z','zero')").execute(&db).await.unwrap();
            sqlx::query("INSERT INTO rods(trip_id,name) VALUES (1,'test')")
                .execute(&db)
                .await
                .unwrap();
            sqlx::query("INSERT INTO intervals(rod_id,starts_at,ends_at) VALUES (1,'2026-01-01T01:00:00Z','2026-01-01T02:00:00Z')").execute(&db).await.unwrap();
            sqlx::query("INSERT INTO trip_positions(trip_id,role,latitude,longitude,origin) VALUES (1,'station',1,2,'manual')").execute(&db).await.unwrap();
            let json: Value = serde_json::from_slice(&build(&db, "json").await.unwrap()).unwrap();
            assert_eq!(json["data"]["trips"][0]["outcome"], "zero");
            assert!(json["data"]["intervals"][0]["pause_minutes"].is_null());
            let geo: Value = serde_json::from_slice(&build(&db, "geojson").await.unwrap()).unwrap();
            assert_eq!(
                geo["features"][0]["geometry"]["coordinates"],
                json!([2.0, 1.0])
            );
            let csv = String::from_utf8(build(&db, "intervals_csv").await.unwrap()).unwrap();
            assert!(csv.contains("\"'=test, uno\""));
            assert!(csv.contains(",\"\",\"\"\r\n"));
        });
    }
}
