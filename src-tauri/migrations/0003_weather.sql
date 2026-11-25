CREATE TABLE forecast_cache (
 latitude REAL NOT NULL, longitude REAL NOT NULL,
 weather TEXT, marine TEXT, retrieved_at TEXT NOT NULL,
 PRIMARY KEY(latitude, longitude)
);
