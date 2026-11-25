use crate::AppError;
use serde::{Deserialize, Serialize};

const RULES: &str = include_str!("../../content/rules.v1.json");
const RIGS: &str = include_str!("../../content/lazio-rigs.v1.json");
const SECTORS: &str = include_str!("../../content/lazio-sectors.v1.json");
const ENGINE_VERSION: &str = "1";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub technique: String,
    pub condition: String,
    pub fixed_rig: Option<String>,
    pub casting_limit_g: Option<f64>,
    pub location: String,
    pub condition_origin: String,
    pub start_at: String,
    pub end_at: String,
    pub target: String,
    #[serde(default)]
    pub forecast_snapshot: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Rule {
    rig_id: String,
    technique: String,
    condition: String,
    sink_g: Option<f64>,
    reason: String,
    risk: String,
    check: String,
    evidence: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RulePackage {
    schema_version: u32,
    content_version: String,
    source: String,
    rules: Vec<Rule>,
}
#[derive(Debug, Serialize)]
pub struct Recommendation {
    rig_id: String,
    reason: String,
    risk: String,
    check: String,
    source: String,
    evidence: String,
}
#[derive(Debug, Serialize)]
pub struct Evaluation {
    recommendations: Vec<Recommendation>,
    warnings: Vec<String>,
    engine_version: String,
    content_version: String,
}
fn invalid(message: &str) -> AppError {
    AppError {
        code: "invalid_scenario",
        message: message.into(),
    }
}
fn load_rules(raw: &str) -> Result<RulePackage, AppError> {
    let pack: RulePackage =
        serde_json::from_str(raw).map_err(|_| invalid("Pacchetto di regole non valido."))?;
    if pack.schema_version != 1
        || pack.content_version.trim().is_empty()
        || pack.source.trim().is_empty()
        || pack.rules.is_empty()
    {
        return Err(invalid("Versione o metadati delle regole non validi."));
    }
    let catalog: serde_json::Value =
        serde_json::from_str(RIGS).map_err(|_| invalid("Catalogo montature non valido."))?;
    let rigs = catalog
        .get("rigs")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| invalid("Catalogo montature non valido."))?;
    let rig_ids: std::collections::HashSet<&str> = rigs
        .iter()
        .filter_map(|item| item.get("id").and_then(serde_json::Value::as_str))
        .collect();
    let mut seen = std::collections::HashSet::new();
    for r in &pack.rules {
        if !rig_ids.contains(r.rig_id.as_str())
            || !["fondo", "bolognese"].contains(&r.technique.as_str())
            || !["calmo", "mosso_lavorabile", "corrente_moderata"].contains(&r.condition.as_str())
            || ![
                "solida",
                "trasferibile_con_cautela",
                "operativa",
                "non_stabilita",
            ]
            .contains(&r.evidence.as_str())
            || r.sink_g.is_some_and(|v| !v.is_finite() || v <= 0.0)
            || r.reason.trim().is_empty()
            || r.risk.trim().is_empty()
            || r.check.trim().is_empty()
            || !seen.insert((&r.rig_id, &r.technique, &r.condition))
        {
            return Err(invalid("Regola incompleta, duplicata o non supportata."));
        }
    }
    Ok(pack)
}
fn evaluate(s: &Scenario) -> Result<Evaluation, AppError> {
    if !["osservazione", "simulazione"].contains(&s.condition_origin.as_str())
        || !crate::time::valid_utc(&s.start_at, true)
        || !crate::time::valid_utc(&s.end_at, true)
        || s.start_at >= s.end_at
        || !["fondo", "bolognese"].contains(&s.technique.as_str())
        || ![
            "calmo",
            "mosso_lavorabile",
            "corrente_moderata",
            "sconosciuto",
        ]
        .contains(&s.condition.as_str())
        || s.forecast_snapshot
            .as_ref()
            .is_some_and(|v| v.to_string().len() > 500_000 || !v.is_object())
        || s.casting_limit_g
            .is_some_and(|v| !v.is_finite() || v <= 0.0)
    {
        return Err(invalid("Controlla tecnica, condizioni e attrezzatura."));
    }
    let sectors: serde_json::Value =
        serde_json::from_str(SECTORS).map_err(|_| invalid("Catalogo settori non valido."))?;
    let sector_list = sectors
        .get("sectors")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| invalid("Catalogo settori non valido."))?;
    let sector_known = sector_list.iter().any(|item| {
        item.get("id").and_then(serde_json::Value::as_str) == Some(s.location.as_str())
    });
    if !s.location.is_empty() && !sector_known {
        return Err(invalid("Settore non disponibile nella ricerca."));
    }
    let pack = load_rules(RULES)?;
    if s.fixed_rig
        .as_ref()
        .is_some_and(|id| !pack.rules.iter().any(|r| &r.rig_id == id))
    {
        return Err(invalid("Montatura non disponibile nel pacchetto."));
    }
    let mut recommendations = Vec::new();
    let mut warnings=vec!["I valori sono indicazioni operative, non probabilità di cattura. Verifica accesso, ordinanze e condizioni sul posto.".into(),"Il peso del piombo non è il peso totale lanciato: verifica anche esca, accessori, filo e componenti.".into()];
    if s.condition == "sconosciuto" {
        warnings.push(
            "Indica condizioni osservate o una simulazione per confrontare gli assetti.".into(),
        );
    }
    for rule in &pack.rules {
        if !sector_known
            || rule.evidence == "non_stabilita"
            || rule.technique != s.technique
            || rule.condition != s.condition
            || s.fixed_rig.as_ref().is_some_and(|id| id != &rule.rig_id)
        {
            continue;
        }
        if let (Some(limit), Some(sink)) = (s.casting_limit_g, rule.sink_g) {
            if limit < sink {
                continue;
            }
        }
        recommendations.push(Recommendation {
            rig_id: rule.rig_id.clone(),
            reason: rule.reason.clone(),
            risk: rule.risk.clone(),
            check: rule.check.clone(),
            source: pack.source.clone(),
            evidence: rule.evidence.clone(),
        });
    }
    if recommendations.is_empty() && s.condition != "sconosciuto" {
        warnings.push("Nessun assetto documentato soddisfa queste scelte: rivedi tecnica, condizioni o vincoli.".into());
    }
    if s.location.trim().is_empty() {
        warnings.push(
            "Settore non specificato: habitat, accesso e divieti restano da verificare.".into(),
        );
    }
    Ok(Evaluation {
        recommendations,
        warnings,
        engine_version: ENGINE_VERSION.into(),
        content_version: pack.content_version,
    })
}
#[tauri::command]
pub fn evaluate_scenario(scenario: Scenario) -> Result<Evaluation, AppError> {
    evaluate(&scenario)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scenario() -> Scenario {
        Scenario {
            technique: "fondo".into(),
            condition: "mosso_lavorabile".into(),
            fixed_rig: None,
            casting_limit_g: Some(100.0),
            location: "santa-severa".into(),
            condition_origin: "simulazione".into(),
            start_at: "2026-01-01T12:00:00Z".into(),
            end_at: "2026-01-01T14:00:00Z".into(),
            target: String::new(),
            forecast_snapshot: None,
        }
    }
    #[test]
    fn respects_capacity_and_fixed_choice() {
        let mut s = scenario();
        assert!(evaluate(&s).unwrap().recommendations.is_empty());
        s.casting_limit_g = Some(150.0);
        s.fixed_rig = Some("C".into());
        let result = evaluate(&s).unwrap();
        assert_eq!(result.recommendations.len(), 1);
        assert_eq!(result.recommendations[0].rig_id, "C");
    }
    #[test]
    fn unsupported_packages_are_rejected() {
        let invalid_package = RULES.replace("\"schema_version\": 1", "\"schema_version\": 99");
        assert!(load_rules(&invalid_package).is_err());
        let invalid_rule = RULES.replace("\"sink_g\": 60", "\"sink_g\": -1");
        assert!(load_rules(&invalid_rule).is_err());
    }
    #[test]
    fn saved_plans_preserve_original_scenario_and_rules() {
        tauri::async_runtime::block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();
            let mut first = scenario();
            first.casting_limit_g = Some(150.0);
            first.fixed_rig = Some("C".into());
            first.forecast_snapshot = Some(
                serde_json::json!({"source":"test", "retrieved_at":"1", "weather":{"hourly":{"time":["synthetic"]}}}),
            );
            persist(&pool, &first).await.unwrap();
            let prior: (String, String, String) =
                sqlx::query_as("SELECT scenario_json,result_json,rules_json FROM plans WHERE id=1")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            first.fixed_rig = Some("D".into());
            first.forecast_snapshot = None;
            persist(&pool, &first).await.unwrap();
            let after: (String, String, String) =
                sqlx::query_as("SELECT scenario_json,result_json,rules_json FROM plans WHERE id=1")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(prior, after);
            assert!(prior.0.contains("synthetic"));
            assert!(prior.1.contains("\"C\""));
            assert!(prior.2.contains("\"rigs\""));
        });
    }
    #[test]
    fn missing_or_uncovered_sector_cannot_produce_recommendations() {
        let mut s = scenario();
        s.location.clear();
        s.casting_limit_g = Some(150.0);
        assert!(evaluate(&s).unwrap().recommendations.is_empty());
        s.location = "altra-regione".into();
        assert!(evaluate(&s).is_err());
    }
    #[test]
    fn unknown_conditions_do_not_invent_a_forecast() {
        let mut s = scenario();
        s.condition = "sconosciuto".into();
        assert!(evaluate(&s).unwrap().recommendations.is_empty());
    }
}

#[derive(Serialize, sqlx::FromRow)]
pub struct SavedPlan {
    pub id: i64,
    pub created_at: String,
    pub scenario_json: String,
    pub result_json: String,
    pub engine_version: String,
    pub content_version: String,
}
async fn persist(pool: &sqlx::SqlitePool, scenario: &Scenario) -> Result<Evaluation, AppError> {
    let result = evaluate(scenario)?;
    let input = serde_json::to_string(scenario).map_err(|_| invalid("Scenario non valido."))?;
    let output = serde_json::to_string(&result).map_err(|_| invalid("Risultato non valido."))?;
    let rules: serde_json::Value =
        serde_json::from_str(RULES).map_err(|_| invalid("Regole non valide."))?;
    let rigs: serde_json::Value =
        serde_json::from_str(RIGS).map_err(|_| invalid("Catalogo non valido."))?;
    let sectors: serde_json::Value =
        serde_json::from_str(SECTORS).map_err(|_| invalid("Catalogo settori non valido."))?;
    let snapshot = serde_json::json!({"rules":rules,"rigs":rigs,"sectors":sectors}).to_string();
    sqlx::query("INSERT INTO plans(scenario_json,result_json,rules_json,engine_version,content_version) VALUES (?,?,?,?,?)")
        .bind(input).bind(output).bind(snapshot).bind(&result.engine_version).bind(&result.content_version)
        .execute(pool).await.map_err(AppError::database)?;
    Ok(result)
}
#[tauri::command]
pub async fn save_plan(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    scenario: Scenario,
) -> Result<Evaluation, AppError> {
    persist(pool.inner(), &scenario).await
}
#[tauri::command]
pub async fn list_plans(
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Vec<SavedPlan>, AppError> {
    sqlx::query_as::<_,SavedPlan>("SELECT id,created_at,scenario_json,result_json,engine_version,content_version FROM plans ORDER BY id DESC").fetch_all(pool.inner()).await.map_err(AppError::database)
}
