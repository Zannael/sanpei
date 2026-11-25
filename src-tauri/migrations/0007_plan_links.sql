CREATE TABLE trip_plans (
 trip_id INTEGER PRIMARY KEY REFERENCES trips(id) ON DELETE CASCADE,
 plan_id INTEGER NOT NULL REFERENCES plans(id),
 followed TEXT NOT NULL DEFAULT 'unknown' CHECK(followed IN ('unknown','yes','partial','no')),
 notes TEXT NOT NULL DEFAULT ''
);
