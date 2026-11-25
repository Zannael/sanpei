CREATE TABLE catches (
 id INTEGER PRIMARY KEY, trip_id INTEGER NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
 occurred_at TEXT NOT NULL, species TEXT NOT NULL, uncertain INTEGER NOT NULL DEFAULT 0,
 length_cm REAL, weight_g REAL, weight_estimated INTEGER NOT NULL DEFAULT 0,
 disposition TEXT NOT NULL CHECK(disposition IN ('released','kept','unknown')),
 CHECK(length_cm IS NULL OR length_cm > 0), CHECK(weight_g IS NULL OR weight_g > 0)
);
CREATE TABLE observations (
 id INTEGER PRIMARY KEY, trip_id INTEGER NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
 occurred_at TEXT NOT NULL, kind TEXT NOT NULL, notes TEXT NOT NULL DEFAULT ''
);
