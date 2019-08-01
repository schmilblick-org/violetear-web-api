-- This file should undo anything in `up.sql`
ALTER TABLE tasks DROP COLUMN status;
ALTER TABLE tasks ADD COLUMN status INTEGER NOT NULL;