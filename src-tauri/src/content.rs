use crate::{planner, AppError};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub source: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bundle {
    pub schema_version: u32,
    pub package_id: String,
    pub content_version: String,
    pub region: String,
    pub review_status: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    pub rules: serde_json::Value,
    pub rigs: serde_json::Value,
    pub sectors: serde_json::Value,
}
fn invalid(message: &str) -> AppError {
    AppError {
        code: "invalid_package",
        message: message.into(),
    }
}
pub fn builtin() -> Result<Bundle, AppError> {
    Ok(Bundle {
        schema_version: 1,
        package_id: "lazio-iniziale".into(),
        content_version: "0.2.0".into(),
        region: "Lazio".into(),
        review_status: "draft".into(),
        parameters: Vec::new(),
        rules: serde_json::from_str(include_str!("../../content/rules.v1.json"))
            .map_err(|_| invalid("Regole iniziali non valide."))?,
        rigs: serde_json::from_str(include_str!("../../content/lazio-rigs.v1.json"))
            .map_err(|_| invalid("Montature iniziali non valide."))?,
        sectors: serde_json::from_str(include_str!("../../content/lazio-sectors.v1.json"))
            .map_err(|_| invalid("Settori iniziali non validi."))?,
    })
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DiagramBranch {
    anchor_cm: f64,
    length_cm: f64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DiagramWeight {
    height_cm: f64,
    mass_g: f64,
    #[serde(default)]
    kind: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Diagram {
    kind: String,
    #[serde(default)]
    main_cm: Option<f64>,
    #[serde(default)]
    terminal_cm: Option<f64>,
    #[serde(default)]
    float_g: Option<f64>,
    #[serde(default)]
    branches: Vec<DiagramBranch>,
    #[serde(default)]
    weights: Vec<DiagramWeight>,
}
fn validate_diagram(value: &serde_json::Value) -> Result<(), AppError> {
    let d: Diagram = serde_json::from_value(value.clone())
        .map_err(|_| invalid("Schema della montatura non valido."))?;
    let positive = |n: f64| n.is_finite() && n > 0.0 && n <= 10000.0;
    let valid = match d.kind.as_str() {
        "bottom" => {
            d.main_cm.is_some_and(positive)
                && d.terminal_cm.is_none()
                && d.float_g.is_none()
                && d.weights.is_empty()
                && !d.branches.is_empty()
                && d.branches.len() <= 2
                && d.branches.iter().all(|b| {
                    b.anchor_cm.is_finite()
                        && b.anchor_cm >= 0.0
                        && b.anchor_cm <= d.main_cm.unwrap_or(0.0)
                        && positive(b.length_cm)
                })
        }
        "sliding" => {
            d.main_cm.is_none()
                && d.float_g.is_none()
                && d.terminal_cm.is_some_and(positive)
                && d.branches.is_empty()
                && d.weights.is_empty()
        }
        "float" => {
            d.main_cm.is_none()
                && d.terminal_cm.is_some_and(positive)
                && d.float_g.is_some_and(positive)
                && d.branches.is_empty()
                && !d.weights.is_empty()
                && d.weights.len() <= 10
                && d.weights.iter().all(|w| {
                    positive(w.height_cm)
                        && positive(w.mass_g)
                        && w.kind
                            .as_ref()
                            .is_none_or(|kind| ["pellet", "torpille"].contains(&kind.as_str()))
                })
        }
        _ => false,
    };
    if !valid {
        return Err(invalid("Misure dello schema non valide o non supportate."));
    }
    Ok(())
}
fn items<'a>(
    value: &'a serde_json::Value,
    key: &str,
    required: &[&str],
) -> Result<std::collections::HashSet<&'a str>, AppError> {
    let allowed_catalog = ["schema_version", "content_version", "status", "source", key];
    if value.as_object().is_none_or(|obj| {
        obj.keys()
            .any(|key| !allowed_catalog.contains(&key.as_str()))
    }) {
        return Err(invalid("Il catalogo contiene campi non supportati."));
    }
    let schema = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64);
    let source = value
        .get("source")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let version = value
        .get("content_version")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let list = value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| invalid("Catalogo incompleto."))?;
    if schema != Some(1) || source.trim().is_empty() || version.trim().is_empty() || list.is_empty()
    {
        return Err(invalid("Metadati del catalogo incompleti."));
    }
    let mut ids = std::collections::HashSet::new();
    for item in list {
        let allowed: &[&str] = if key == "rigs" {
            &[
                "id",
                "name",
                "use",
                "detail",
                "caution",
                "components",
                "adjustments",
                "diagram",
            ]
        } else {
            &["id", "name", "habitat", "techniques", "caution"]
        };
        if item
            .as_object()
            .is_none_or(|obj| obj.keys().any(|k| !allowed.contains(&k.as_str())))
        {
            return Err(invalid("La scheda contiene campi non supportati."));
        }
        let id = item
            .get("id")
            .and_then(serde_json::Value::as_str)
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| invalid("Identificativo mancante."))?;
        if !ids.insert(id)
            || required.iter().any(|field| {
                item.get(field)
                    .and_then(serde_json::Value::as_str)
                    .is_none_or(|x| x.trim().is_empty())
            })
        {
            return Err(invalid("Scheda incompleta o identificativo duplicato."));
        }
        if key == "rigs" {
            if let Some(diagram) = item.get("diagram") {
                validate_diagram(diagram)?;
            }
        }
        if key == "rigs"
            && item
                .get("components")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|a| {
                    a.is_empty()
                        || a.iter()
                            .any(|x| x.as_str().is_none_or(|s| s.trim().is_empty()))
                })
        {
            return Err(invalid("Componenti della montatura mancanti."));
        }
    }
    Ok(ids)
}
pub fn validate(bundle: &Bundle, imported: bool) -> Result<(), AppError> {
    if bundle.schema_version != 1
        || bundle.region != "Lazio"
        || bundle.package_id.is_empty()
        || bundle.package_id.len() > 80
        || !bundle
            .package_id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        || bundle.content_version.trim().is_empty()
        || bundle.content_version.len() > 40
        || !["draft", "reviewed"].contains(&bundle.review_status.as_str())
        || (imported && bundle.review_status != "reviewed")
    {
        return Err(invalid(
            "Versione, regione o stato di revisione non supportati.",
        ));
    }
    items(
        &bundle.rigs,
        "rigs",
        &["name", "use", "detail", "caution", "adjustments"],
    )?;
    items(
        &bundle.sectors,
        "sectors",
        &["name", "habitat", "techniques", "caution"],
    )?;
    let mut ids = std::collections::HashSet::new();
    for param in &bundle.parameters {
        if param.id.is_empty()
            || param.id.len() > 80
            || !param
                .id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
            || !ids.insert(&param.id)
            || param.label.trim().is_empty()
            || param.source.trim().is_empty()
            || !["number", "boolean", "category"].contains(&param.kind.as_str())
            || param.minimum.is_some_and(|x| !x.is_finite())
            || param.maximum.is_some_and(|x| !x.is_finite())
            || matches!((param.minimum,param.maximum),(Some(a),Some(b)) if a>b)
        {
            return Err(invalid("Definizione del parametro non valida."));
        }
        if param.kind != "number"
            && (param.minimum.is_some() || param.maximum.is_some() || param.unit.is_some())
        {
            return Err(invalid(
                "Unità e intervalli sono riservati ai parametri numerici.",
            ));
        }
        let options: std::collections::HashSet<&str> =
            param.options.iter().map(String::as_str).collect();
        if (param.kind == "category"
            && (param.options.is_empty()
                || options.len() != param.options.len()
                || param.options.iter().any(|x| x.trim().is_empty())))
            || (param.kind != "category" && !param.options.is_empty())
        {
            return Err(invalid("Opzioni del parametro non valide."));
        }
    }
    planner::validate_rules(&bundle.rules, &bundle.rigs, &bundle.parameters)?;
    Ok(())
}
pub async fn active(pool: &SqlitePool) -> Result<Bundle, AppError> {
    let data: Option<String> = sqlx::query_scalar(
        "SELECT bundle_json FROM research_packages WHERE region='Lazio' AND active=1",
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::database)?;
    let bundle = match data {
        Some(raw) => {
            serde_json::from_str(&raw).map_err(|_| invalid("Pacchetto attivo non leggibile."))?
        }
        None => builtin()?,
    };
    validate(&bundle, false)?;
    Ok(bundle)
}
#[tauri::command]
pub async fn active_package(pool: tauri::State<'_, SqlitePool>) -> Result<Bundle, AppError> {
    active(pool.inner()).await
}
async fn store(pool: &SqlitePool, bundle: &Bundle) -> Result<(), AppError> {
    validate(bundle, true)?;
    let initial = builtin()?;
    if bundle.package_id == initial.package_id && bundle.content_version == initial.content_version
    {
        return Err(invalid(
            "Identificativo e versione sono riservati alla ricerca iniziale.",
        ));
    }
    let json =
        serde_json::to_string(bundle).map_err(|_| invalid("Pacchetto non serializzabile."))?;
    let mut tx = pool.begin().await.map_err(AppError::database)?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM research_packages WHERE package_id=? AND content_version=?)",
    )
    .bind(&bundle.package_id)
    .bind(&bundle.content_version)
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::database)?;
    if exists {
        return Err(invalid("Questa versione è già stata importata."));
    }
    sqlx::query("UPDATE research_packages SET active=0 WHERE region=?")
        .bind(&bundle.region)
        .execute(&mut *tx)
        .await
        .map_err(AppError::database)?;
    sqlx::query("INSERT INTO research_packages(package_id,content_version,region,bundle_json,active) VALUES (?,?,?,?,1)").bind(&bundle.package_id).bind(&bundle.content_version).bind(&bundle.region).bind(json).execute(&mut *tx).await.map_err(AppError::database)?;
    tx.commit().await.map_err(AppError::database)?;
    Ok(())
}
#[tauri::command]
pub async fn import_package(pool: tauri::State<'_, SqlitePool>) -> Result<bool, AppError> {
    let Some(file) = rfd::AsyncFileDialog::new()
        .set_title("Importa un pacchetto di ricerca revisionato")
        .add_filter("Pacchetto JSON", &["json"])
        .pick_file()
        .await
    else {
        return Ok(false);
    };
    if std::fs::metadata(file.path())
        .map_err(|_| invalid("Impossibile leggere il file."))?
        .len()
        > 2_000_000
    {
        return Err(invalid("Il pacchetto supera la dimensione consentita."));
    }
    let bytes = std::fs::read(file.path()).map_err(|_| invalid("Impossibile leggere il file."))?;
    if bytes.len() > 2_000_000 {
        return Err(invalid("Il pacchetto supera la dimensione consentita."));
    }
    let bundle: Bundle =
        serde_json::from_slice(&bytes).map_err(|_| invalid("Formato del pacchetto non valido."))?;
    validate(&bundle, true)?;
    let confirm=rfd::AsyncMessageDialog::new().set_title("Attivare la ricerca?").set_description(format!("Pacchetto: {}\nVersione: {}\nRegione: {}\nLa versione corrente resterà disponibile nello storico.",bundle.package_id,bundle.content_version,bundle.region)).set_buttons(rfd::MessageButtons::YesNo).show().await;
    if confirm != rfd::MessageDialogResult::Yes {
        return Ok(false);
    }
    store(pool.inner(), &bundle).await?;
    Ok(true)
}
#[derive(Serialize, sqlx::FromRow)]
pub struct PackageInfo {
    package_id: String,
    content_version: String,
    region: String,
    active: bool,
    imported_at: String,
}
#[tauri::command]
pub async fn list_packages(
    pool: tauri::State<'_, SqlitePool>,
) -> Result<Vec<PackageInfo>, AppError> {
    sqlx::query_as::<_,PackageInfo>("SELECT package_id,content_version,region,active,imported_at FROM research_packages ORDER BY imported_at DESC, rowid DESC").fetch_all(pool.inner()).await.map_err(AppError::database)
}
#[tauri::command]
pub async fn activate_package(
    pool: tauri::State<'_, SqlitePool>,
    package_id: String,
    content_version: String,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(AppError::database)?;

    let raw:Option<String>=sqlx::query_scalar("SELECT bundle_json FROM research_packages WHERE package_id=? AND content_version=? AND region='Lazio'").bind(&package_id).bind(&content_version).fetch_optional(&mut *tx).await.map_err(AppError::database)?;
    let bundle: Bundle =
        serde_json::from_str(&raw.ok_or_else(|| invalid("Pacchetto non trovato."))?)
            .map_err(|_| invalid("Pacchetto non leggibile."))?;
    validate(&bundle, true)?;
    sqlx::query("UPDATE research_packages SET active=0 WHERE region='Lazio'")
        .execute(&mut *tx)
        .await
        .map_err(AppError::database)?;
    sqlx::query("UPDATE research_packages SET active=1 WHERE package_id=? AND content_version=?")
        .bind(package_id)
        .bind(content_version)
        .execute(&mut *tx)
        .await
        .map_err(AppError::database)?;

    tx.commit().await.map_err(AppError::database)?;
    Ok(())
}

#[tauri::command]
pub async fn activate_builtin(pool: tauri::State<'_, SqlitePool>) -> Result<(), AppError> {
    sqlx::query("UPDATE research_packages SET active=0 WHERE region='Lazio'")
        .execute(pool.inner())
        .await
        .map_err(AppError::database)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_and_switches_packages_without_overwriting_versions() {
        tauri::async_runtime::block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();
            let mut first = builtin().unwrap();
            first.package_id = "test".into();
            first.content_version = "1".into();
            first.review_status = "reviewed".into();
            store(&pool, &first).await.unwrap();
            let scenario = planner::Scenario {
                technique: "fondo".into(),
                condition: "calmo".into(),
                fixed_rig: Some("A".into()),
                casting_limit_g: None,
                location: "santa-severa".into(),
                condition_origin: "simulazione".into(),
                start_at: "2026-01-01T12:00:00Z".into(),
                end_at: "2026-01-01T14:00:00Z".into(),
                target: String::new(),
                forecast_snapshot: None,
                previous_plan_id: None,
                extra_parameters: Default::default(),
            };
            planner::persist(&pool, &scenario).await.unwrap();
            let previous: (String, String) =
                sqlx::query_as("SELECT result_json,rules_json FROM plans WHERE id=1")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            let mut duplicate = first.clone();
            duplicate.rules["rules"][0]["reason"] = serde_json::json!("changed");
            assert!(store(&pool, &duplicate).await.is_err());
            assert_eq!(active(&pool).await.unwrap().content_version, "1");
            let mut invalid = first.clone();
            invalid.content_version = "2".into();
            invalid.rules["rules"][0]["rig_id"] = serde_json::json!("missing");
            assert!(store(&pool, &invalid).await.is_err());
            let mut malformed_diagram = first.clone();
            malformed_diagram.content_version = "bad-diagram".into();
            malformed_diagram.rigs["rigs"][0]["diagram"]["branches"][0]["anchor_cm"] =
                serde_json::json!(9999);
            assert!(store(&pool, &malformed_diagram).await.is_err());
            let mut unsupported = first.clone();
            unsupported.content_version = "2".into();
            unsupported.sectors["sectors"][0]["unsupported"] = serde_json::json!(true);
            assert!(store(&pool, &unsupported).await.is_err());
            assert_eq!(active(&pool).await.unwrap().content_version, "1");
            let mut second = first.clone();
            second.content_version = "2".into();
            second.rules["rules"][0]["reason"] = serde_json::json!("motivo aggiornato");
            store(&pool, &second).await.unwrap();
            let updated = planner::evaluate(&scenario, &active(&pool).await.unwrap()).unwrap();
            assert!(serde_json::to_string(&updated)
                .unwrap()
                .contains("motivo aggiornato"));
            let preserved: (String, String) =
                sqlx::query_as("SELECT result_json,rules_json FROM plans WHERE id=1")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(previous, preserved);
            assert_eq!(active(&pool).await.unwrap().content_version, "2");
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM research_packages")
                    .fetch_one(&pool)
                    .await
                    .unwrap(),
                2
            );
            let mut third = second.clone();
            third.content_version = "3".into();
            third.parameters.push(Parameter {
                id: "chiarezza".into(),
                label: "Chiarezza".into(),
                kind: "category".into(),
                source: "fonte di prova".into(),
                unit: None,
                options: vec!["limpida".into(), "torbida".into()],
                minimum: None,
                maximum: None,
            });
            third.rules["rules"][0]["predicates"] =
                serde_json::json!([{"parameter":"chiarezza","operator":"eq","value":"limpida"}]);
            let parsed: Bundle =
                serde_json::from_str(&serde_json::to_string(&third).unwrap()).unwrap();
            store(&pool, &parsed).await.unwrap();
            let missing = serde_json::to_value(
                planner::evaluate(&scenario, &active(&pool).await.unwrap()).unwrap(),
            )
            .unwrap();
            assert!(missing["recommendations"].as_array().unwrap().is_empty());
            let mut filled = scenario.clone();
            filled
                .extra_parameters
                .insert("chiarezza".into(), serde_json::json!("limpida"));
            let evaluated = serde_json::to_value(
                planner::evaluate(&filled, &active(&pool).await.unwrap()).unwrap(),
            )
            .unwrap();
            assert_eq!(evaluated["recommendations"].as_array().unwrap().len(), 1);
            let after: (String, String) =
                sqlx::query_as("SELECT result_json,rules_json FROM plans WHERE id=1")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(previous, after);
        });
    }
}
