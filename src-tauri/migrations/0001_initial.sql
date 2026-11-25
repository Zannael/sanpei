CREATE TABLE trips (
 id INTEGER PRIMARY KEY,
 title TEXT NOT NULL,
 starts_at TEXT NOT NULL,
 location TEXT NOT NULL DEFAULT '',
 notes TEXT NOT NULL DEFAULT '',
 outcome TEXT NOT NULL DEFAULT 'incomplete' CHECK(outcome IN ('incomplete','zero','catches')),
 created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE rods (id INTEGER PRIMARY KEY, trip_id INTEGER NOT NULL REFERENCES trips(id) ON DELETE CASCADE, name TEXT NOT NULL);
CREATE TABLE intervals (id INTEGER PRIMARY KEY, rod_id INTEGER NOT NULL REFERENCES rods(id) ON DELETE CASCADE, starts_at TEXT NOT NULL, ends_at TEXT NOT NULL, rig TEXT NOT NULL DEFAULT '', bait TEXT NOT NULL DEFAULT '', CHECK(starts_at < ends_at));
CREATE TRIGGER intervals_no_overlap BEFORE INSERT ON intervals WHEN EXISTS (SELECT 1 FROM intervals WHERE rod_id = NEW.rod_id AND NEW.starts_at < ends_at AND NEW.ends_at > starts_at) BEGIN SELECT RAISE(ABORT, 'interval_overlap'); END;
CREATE TRIGGER intervals_no_overlap_update BEFORE UPDATE ON intervals WHEN EXISTS (SELECT 1 FROM intervals WHERE rod_id = NEW.rod_id AND id != NEW.id AND NEW.starts_at < ends_at AND NEW.ends_at > starts_at) BEGIN SELECT RAISE(ABORT, 'interval_overlap'); END;
