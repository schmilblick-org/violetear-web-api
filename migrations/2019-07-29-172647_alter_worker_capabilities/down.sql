-- This file should undo anything in `up.sql`
ALTER TABLE worker_capabilities ADD COLUMN kind TEXT NOT NULL;
ALTER TABLE worker_capabilities ADD COLUMN value TEXT NOT NULL;
ALTER TABLE worker_capabilities DROP COLUMN profile_id; 