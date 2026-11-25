CREATE TABLE research_packages (
 package_id TEXT NOT NULL,
 content_version TEXT NOT NULL,
 region TEXT NOT NULL,
 bundle_json TEXT NOT NULL CHECK(json_valid(bundle_json)),
 active INTEGER NOT NULL CHECK(active IN (0,1)),
 imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
 PRIMARY KEY(package_id,content_version)
);
CREATE UNIQUE INDEX research_one_active ON research_packages(region) WHERE active=1;
