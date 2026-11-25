use crate::AppError;
use serde::{Deserialize, Serialize};

const ENGINE_VERSION: &str = "3";
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
    #[serde(default)]
    pub previous_plan_id: Option<i64>,
    #[serde(default)]
    pub extra_parameters: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Predicate {
    parameter: String,
    operator: String,
    value: serde_json::Value,
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
    #[serde(default)]
    predicates: Vec<Predicate>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RulePackage {
    schema_version: u32,
    content_version: String,
    source: String,
    rules: Vec<Rule>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    rig_id: String,
    reason: String,
    risk: String,
    check: String,
    source: String,
    evidence: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Comparison {
    previous_plan_id: i64,
    added: Vec<String>,
    removed: Vec<String>,
    unchanged: Vec<String>,
    changed_inputs: Vec<String>,
    content_changed: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluation {
    recommendations: Vec<Recommendation>,
    warnings: Vec<String>,
    engine_version: String,
    content_version: String,
    #[serde(default)]
    comparison: Option<Comparison>,
}
fn invalid(message: &str) -> AppError {
    AppError {
        code: "invalid_scenario",
        message: message.into(),
    }
}
fn value_valid(parameter: &crate::content::Parameter, value: &serde_json::Value) -> bool {
    match parameter.kind.as_str() {
        "number" => value.as_f64().is_some_and(|n| {
            n.is_finite()
                && parameter.minimum.is_none_or(|min| n >= min)
                && parameter.maximum.is_none_or(|max| n <= max)
        }),
        "boolean" => value.is_boolean(),
        "category" => value
            .as_str()
            .is_some_and(|v| parameter.options.iter().any(|o| o == v)),
        _ => false,
    }
}
fn load_rules(
    raw: &str,
    catalog: &serde_json::Value,
    parameters: &[crate::content::Parameter],
) -> Result<RulePackage, AppError> {
    let pack: RulePackage =
        serde_json::from_str(raw).map_err(|_| invalid("Pacchetto di regole non valido."))?;
    if pack.schema_version != 1
        || pack.content_version.trim().is_empty()
        || pack.source.trim().is_empty()
        || pack.rules.is_empty()
    {
        return Err(invalid("Versione o metadati delle regole non validi."));
    }
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
    for rule in &pack.rules {
        for predicate in &rule.predicates {
            let param = parameters
                .iter()
                .find(|p| p.id == predicate.parameter)
                .ok_or_else(|| invalid("La regola usa un parametro non definito."))?;
            let supported = match param.kind.as_str() {
                "number" => {
                    ["eq", "neq", "lt", "lte", "gt", "gte"].contains(&predicate.operator.as_str())
                }
                "boolean" | "category" => ["eq", "neq"].contains(&predicate.operator.as_str()),
                _ => false,
            };
            if !supported || !value_valid(param, &predicate.value) {
                return Err(invalid(
                    "Condizione del parametro non valida o non supportata.",
                ));
            }
        }
    }
    Ok(pack)
}
pub(crate) fn validate_rules(
    rules: &serde_json::Value,
    rigs: &serde_json::Value,
    parameters: &[crate::content::Parameter],
) -> Result<(), AppError> {
    let raw = serde_json::to_string(rules).map_err(|_| invalid("Regole non valide."))?;
    load_rules(&raw, rigs, parameters).map(|_| ())
}
pub(crate) fn evaluate(
    s: &Scenario,
    bundle: &crate::content::Bundle,
) -> Result<Evaluation, AppError> {
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
    let sectors = &bundle.sectors;
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
    let raw_rules =
        serde_json::to_string(&bundle.rules).map_err(|_| invalid("Regole non valide."))?;
    let pack = load_rules(&raw_rules, &bundle.rigs, &bundle.parameters)?;
    for (id, value) in &s.extra_parameters {
        let param = bundle
            .parameters
            .iter()
            .find(|p| &p.id == id)
            .ok_or_else(|| invalid("Parametro non disponibile nel pacchetto."))?;
        if !value_valid(param, value) {
            return Err(invalid("Valore di un parametro non valido."));
        }
    }
    let mut missing = std::collections::BTreeSet::new();
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
        let mut matched = true;
        for predicate in &rule.predicates {
            let Some(actual) = s.extra_parameters.get(&predicate.parameter) else {
                missing.insert(predicate.parameter.clone());
                matched = false;
                continue;
            };
            let matches = match predicate.operator.as_str() {
                "eq" => actual
                    .as_f64()
                    .zip(predicate.value.as_f64())
                    .map_or_else(|| actual == &predicate.value, |(a, b)| a == b),
                "neq" => actual
                    .as_f64()
                    .zip(predicate.value.as_f64())
                    .map_or_else(|| actual != &predicate.value, |(a, b)| a != b),
                "lt" => actual
                    .as_f64()
                    .zip(predicate.value.as_f64())
                    .is_some_and(|(a, b)| a < b),
                "lte" => actual
                    .as_f64()
                    .zip(predicate.value.as_f64())
                    .is_some_and(|(a, b)| a <= b),
                "gt" => actual
                    .as_f64()
                    .zip(predicate.value.as_f64())
                    .is_some_and(|(a, b)| a > b),
                "gte" => actual
                    .as_f64()
                    .zip(predicate.value.as_f64())
                    .is_some_and(|(a, b)| a >= b),
                _ => false,
            };
            if !matches {
                matched = false;
            }
        }
        if !matched {
            continue;
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
    if !missing.is_empty() {
        warnings.push(format!(
            "Dati mancanti per verificare alcune proposte: {}.",
            missing.into_iter().collect::<Vec<_>>().join(", ")
        ));
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
        content_version: bundle.content_version.clone(),
        comparison: None,
    })
}
async fn add_comparison(
    pool: &sqlx::SqlitePool,
    scenario: &Scenario,
    bundle: &crate::content::Bundle,
    result: &mut Evaluation,
) -> Result<(), AppError> {
    let Some(id) = scenario.previous_plan_id else {
        return Ok(());
    };
    let previous: Option<(String, String, String)> =
        sqlx::query_as("SELECT scenario_json,result_json,rules_json FROM plans WHERE id=?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(AppError::database)?;
    let (old_input, old_result, old_content) =
        previous.ok_or_else(|| invalid("Piano di riferimento non trovato."))?;
    let old: Scenario = serde_json::from_str(&old_input)
        .map_err(|_| invalid("Scenario precedente non leggibile."))?;
    let old_result: Evaluation = serde_json::from_str(&old_result)
        .map_err(|_| invalid("Risultato precedente non leggibile."))?;
    let old_content: serde_json::Value = serde_json::from_str(&old_content)
        .map_err(|_| invalid("Contenuti precedenti non leggibili."))?;
    let old_package = old_content
        .get("package_id")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("lazio-iniziale");
    let old_ids: std::collections::BTreeSet<String> = old_result
        .recommendations
        .iter()
        .map(|r| r.rig_id.clone())
        .collect();
    let new_ids: std::collections::BTreeSet<String> = result
        .recommendations
        .iter()
        .map(|r| r.rig_id.clone())
        .collect();
    let mut changed = Vec::new();
    if old.condition != scenario.condition {
        changed.push("condizioni".into());
    }
    if old.condition_origin != scenario.condition_origin {
        changed.push("origine delle condizioni".into());
    }
    if old.technique != scenario.technique {
        changed.push("tecnica".into());
    }
    if old.fixed_rig != scenario.fixed_rig {
        changed.push("montatura fissata".into());
    }
    if old.casting_limit_g != scenario.casting_limit_g {
        changed.push("potenza della canna".into());
    }
    if old.location != scenario.location {
        changed.push("settore".into());
    }
    if old.start_at != scenario.start_at || old.end_at != scenario.end_at {
        changed.push("finestra oraria".into());
    }
    if old.target != scenario.target {
        changed.push("specie annotata".into());
    }
    if old.extra_parameters != scenario.extra_parameters {
        changed.push("parametri aggiuntivi".into());
    }
    if old.forecast_snapshot != scenario.forecast_snapshot {
        changed.push("dati meteo allegati".into());
    }
    result.comparison = Some(Comparison {
        previous_plan_id: id,
        added: new_ids.difference(&old_ids).cloned().collect(),
        removed: old_ids.difference(&new_ids).cloned().collect(),
        unchanged: new_ids.intersection(&old_ids).cloned().collect(),
        changed_inputs: changed,
        content_changed: old_package != bundle.package_id
            || old_result.content_version != bundle.content_version
            || old_result.engine_version != result.engine_version
            || old_content
                != serde_json::to_value(bundle).map_err(|_| invalid("Pacchetto non valido."))?,
    });
    Ok(())
}

#[tauri::command]
pub async fn evaluate_scenario(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    scenario: Scenario,
) -> Result<Evaluation, AppError> {
    let bundle = crate::content::active(pool.inner()).await?;
    let mut result = evaluate(&scenario, &bundle)?;
    add_comparison(pool.inner(), &scenario, &bundle, &mut result).await?;
    Ok(result)
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
            previous_plan_id: None,
            extra_parameters: Default::default(),
        }
    }
    #[test]
    fn respects_capacity_and_fixed_choice() {
        let mut s = scenario();
        assert!(evaluate(&s, &crate::content::builtin().unwrap())
            .unwrap()
            .recommendations
            .is_empty());
        s.casting_limit_g = Some(150.0);
        s.fixed_rig = Some("C".into());
        let result = evaluate(&s, &crate::content::builtin().unwrap()).unwrap();
        assert_eq!(result.recommendations.len(), 1);
        assert_eq!(result.recommendations[0].rig_id, "C");
    }
    #[test]
    fn earlier_snapshots_remain_readable() {
        let original = scenario();
        let mut old = serde_json::to_value(&original).unwrap();
        let object = old.as_object_mut().unwrap();
        object.remove("extra_parameters");
        object.remove("previous_plan_id");
        object.remove("forecast_snapshot");
        let restored: Scenario = serde_json::from_value(old).unwrap();
        assert!(restored.extra_parameters.is_empty());
        assert!(restored.previous_plan_id.is_none());
        assert!(restored.forecast_snapshot.is_none());
        let mut output =
            serde_json::to_value(evaluate(&original, &crate::content::builtin().unwrap()).unwrap())
                .unwrap();
        output.as_object_mut().unwrap().remove("comparison");
        let restored: Evaluation = serde_json::from_value(output).unwrap();
        assert!(restored.comparison.is_none());
    }
    #[test]
    fn typed_additional_parameters_are_safe_and_explain_missing_values() {
        let mut bundle = crate::content::builtin().unwrap();
        bundle.parameters.push(crate::content::Parameter {
            id: "torbidita".into(),
            label: "Torbidità osservata".into(),
            kind: "number".into(),
            source: "fonte di prova".into(),
            unit: Some("unità".into()),
            options: vec![],
            minimum: Some(0.0),
            maximum: Some(10.0),
        });
        bundle.rules["rules"][0]["predicates"] =
            serde_json::json!([{"parameter":"torbidita","operator":"gte","value":3}]);
        crate::content::validate(&bundle, false).unwrap();
        let mut s = scenario();
        s.condition = "calmo".into();
        s.fixed_rig = Some("A".into());
        s.casting_limit_g = None;
        let missing = evaluate(&s, &bundle).unwrap();
        assert!(missing.recommendations.is_empty());
        assert!(missing.warnings.iter().any(|w| w.contains("torbidita")));
        s.extra_parameters
            .insert("torbidita".into(), serde_json::json!(2));
        assert!(evaluate(&s, &bundle).unwrap().recommendations.is_empty());
        s.extra_parameters
            .insert("torbidita".into(), serde_json::json!(4));
        assert_eq!(evaluate(&s, &bundle).unwrap().recommendations.len(), 1);
        s.extra_parameters
            .insert("torbidita".into(), serde_json::json!("4"));
        assert!(evaluate(&s, &bundle).is_err());
        s.extra_parameters
            .insert("torbidita".into(), serde_json::json!(4));
        bundle.rules["rules"][0]["predicates"][0]["operator"] = serde_json::json!("eq");
        bundle.rules["rules"][0]["predicates"][0]["value"] = serde_json::json!(4.0);
        assert_eq!(evaluate(&s, &bundle).unwrap().recommendations.len(), 1);
        bundle.rules["rules"][0]["predicates"][0]["operator"] = serde_json::json!("exec");
        assert!(crate::content::validate(&bundle, false).is_err());
    }
    #[test]
    fn unsupported_packages_are_rejected() {
        let bundle = crate::content::builtin().unwrap();
        let raw = serde_json::to_string_pretty(&bundle.rules).unwrap();
        let invalid_package = raw.replace("\"schema_version\": 1", "\"schema_version\": 99");
        assert!(load_rules(&invalid_package, &bundle.rigs, &bundle.parameters).is_err());
        let invalid_rule = raw.replace("\"sink_g\": 60", "\"sink_g\": -1");
        assert!(load_rules(&invalid_rule, &bundle.rigs, &bundle.parameters).is_err());
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
        assert!(evaluate(&s, &crate::content::builtin().unwrap())
            .unwrap()
            .recommendations
            .is_empty());
        s.location = "altra-regione".into();
        assert!(evaluate(&s, &crate::content::builtin().unwrap()).is_err());
    }
    #[test]
    fn adaptation_compares_without_mutating_the_original() {
        tauri::async_runtime::block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();
            let mut first = scenario();
            first.condition = "calmo".into();
            first.casting_limit_g = None;
            persist(&pool, &first).await.unwrap();
            let before: String = sqlx::query_scalar("SELECT result_json FROM plans WHERE id=1")
                .fetch_one(&pool)
                .await
                .unwrap();
            let mut next = first.clone();
            next.condition = "mosso_lavorabile".into();
            next.condition_origin = "osservazione".into();
            next.previous_plan_id = Some(1);
            let evaluation = persist(&pool, &next).await.unwrap();
            let comparison = evaluation.comparison.unwrap();
            assert_eq!(comparison.added, vec!["C", "D"]);
            assert_eq!(comparison.removed, vec!["A", "B"]);
            assert!(comparison
                .changed_inputs
                .contains(&"condizioni".to_string()));
            assert!(!comparison.content_changed);
            let mut altered = crate::content::builtin().unwrap();
            altered.rigs["rigs"][0]["name"] = serde_json::json!("nome cambiato");
            let mut compared = evaluate(&next, &altered).unwrap();
            add_comparison(&pool, &next, &altered, &mut compared)
                .await
                .unwrap();
            assert!(compared.comparison.unwrap().content_changed);
            let after: String = sqlx::query_scalar("SELECT result_json FROM plans WHERE id=1")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(before, after);
            next.previous_plan_id = Some(999);
            assert!(persist(&pool, &next).await.is_err());
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM plans")
                    .fetch_one(&pool)
                    .await
                    .unwrap(),
                2
            );
        });
    }
    #[test]
    fn saved_plan_creates_linked_trip_atomically() {
        tauri::async_runtime::block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();
            persist(&pool, &scenario()).await.unwrap();
            let id = create_trip(&pool, 1).await.unwrap();
            let link: (i64, String) =
                sqlx::query_as("SELECT plan_id,followed FROM trip_plans WHERE trip_id=?")
                    .bind(id)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(link, (1, "unknown".into()));
            let location: String = sqlx::query_scalar("SELECT location FROM trips WHERE id=?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(location, "Santa Severa");
            assert!(create_trip(&pool, 999).await.is_err());
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM trips")
                    .fetch_one(&pool)
                    .await
                    .unwrap(),
                1
            );
        });
    }
    #[test]
    fn unknown_conditions_do_not_invent_a_forecast() {
        let mut s = scenario();
        s.condition = "sconosciuto".into();
        assert!(evaluate(&s, &crate::content::builtin().unwrap())
            .unwrap()
            .recommendations
            .is_empty());
    }
}

#[derive(Serialize, sqlx::FromRow)]
pub struct SavedPlan {
    pub id: i64,
    pub created_at: String,
    pub scenario_json: String,
    pub result_json: String,
    pub rules_json: String,
    pub engine_version: String,
    pub content_version: String,
}
pub(crate) async fn persist(
    pool: &sqlx::SqlitePool,
    scenario: &Scenario,
) -> Result<Evaluation, AppError> {
    let bundle = crate::content::active(pool).await?;
    let mut result = evaluate(scenario, &bundle)?;
    add_comparison(pool, scenario, &bundle, &mut result).await?;
    let input = serde_json::to_string(scenario).map_err(|_| invalid("Scenario non valido."))?;
    let output = serde_json::to_string(&result).map_err(|_| invalid("Risultato non valido."))?;
    let snapshot = serde_json::to_string(&bundle).map_err(|_| invalid("Pacchetto non valido."))?;
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
    sqlx::query_as::<_,SavedPlan>("SELECT id,created_at,scenario_json,result_json,rules_json,engine_version,content_version FROM plans ORDER BY id DESC").fetch_all(pool.inner()).await.map_err(AppError::database)
}

async fn create_trip(pool: &sqlx::SqlitePool, plan_id: i64) -> Result<i64, AppError> {
    let row: Option<(String, String)> =
        sqlx::query_as("SELECT scenario_json,rules_json FROM plans WHERE id=?")
            .bind(plan_id)
            .fetch_optional(pool)
            .await
            .map_err(AppError::database)?;
    let (scenario_json, contents) = row.ok_or_else(|| invalid("Piano non trovato."))?;
    let scenario: Scenario = serde_json::from_str(&scenario_json)
        .map_err(|_| invalid("Piano salvato non leggibile."))?;
    if !crate::time::valid_utc(&scenario.start_at, true) {
        return Err(invalid("La data del piano non è valida."));
    }
    let catalog: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|_| invalid("Contenuti del piano non leggibili."))?;
    let location = catalog
        .get("sectors")
        .and_then(|x| x.get("sectors"))
        .and_then(serde_json::Value::as_array)
        .and_then(|a| {
            a.iter().find(|item| {
                item.get("id").and_then(serde_json::Value::as_str)
                    == Some(scenario.location.as_str())
            })
        })
        .and_then(|item| item.get("name"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let mut tx = pool.begin().await.map_err(AppError::database)?;
    let inserted = sqlx::query("INSERT INTO trips(title,starts_at,location) VALUES (?,?,?)")
        .bind(format!("Uscita dal piano {plan_id}"))
        .bind(&scenario.start_at)
        .bind(location)
        .execute(&mut *tx)
        .await
        .map_err(AppError::database)?;
    let trip_id = inserted.last_insert_rowid();
    sqlx::query("INSERT INTO trip_plans(trip_id,plan_id) VALUES (?,?)")
        .bind(trip_id)
        .bind(plan_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::database)?;
    tx.commit().await.map_err(AppError::database)?;
    Ok(trip_id)
}

#[tauri::command]
pub async fn create_trip_from_plan(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    plan_id: i64,
) -> Result<i64, AppError> {
    create_trip(pool.inner(), plan_id).await
}
