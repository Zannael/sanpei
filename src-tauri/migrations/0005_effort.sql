ALTER TABLE intervals ADD COLUMN pause_minutes INTEGER CHECK(pause_minutes IS NULL OR pause_minutes >= 0);
ALTER TABLE intervals ADD COLUMN ineffective_minutes INTEGER CHECK(ineffective_minutes IS NULL OR ineffective_minutes >= 0);
CREATE TRIGGER intervals_effort_insert BEFORE INSERT ON intervals
WHEN (coalesce(NEW.pause_minutes,0) + coalesce(NEW.ineffective_minutes,0)) * 60 > unixepoch(NEW.ends_at) - unixepoch(NEW.starts_at)
BEGIN SELECT RAISE(ABORT, 'invalid_effort'); END;
CREATE TRIGGER intervals_effort_update BEFORE UPDATE ON intervals
WHEN (coalesce(NEW.pause_minutes,0) + coalesce(NEW.ineffective_minutes,0)) * 60 > unixepoch(NEW.ends_at) - unixepoch(NEW.starts_at)
BEGIN SELECT RAISE(ABORT, 'invalid_effort'); END;
