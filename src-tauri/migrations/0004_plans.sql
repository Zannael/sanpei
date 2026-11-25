CREATE TABLE plans (
 id INTEGER PRIMARY KEY,
 created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
 scenario_json TEXT NOT NULL CHECK(json_valid(scenario_json)),
 result_json TEXT NOT NULL CHECK(json_valid(result_json)),
 rules_json TEXT NOT NULL CHECK(json_valid(rules_json)),
 engine_version TEXT NOT NULL,
 content_version TEXT NOT NULL
);
