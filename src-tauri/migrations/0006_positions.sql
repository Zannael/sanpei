CREATE TABLE trip_positions (
 id INTEGER PRIMARY KEY,
 trip_id INTEGER NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
 role TEXT NOT NULL CHECK(role IN ('station','cast')),
 latitude REAL NOT NULL CHECK(latitude BETWEEN -90 AND 90),
 longitude REAL NOT NULL CHECK(longitude BETWEEN -180 AND 180),
 origin TEXT NOT NULL CHECK(origin IN ('manual','map','gps')),
 accuracy_m REAL CHECK(accuracy_m IS NULL OR accuracy_m >= 0),
 UNIQUE(trip_id,role)
);
