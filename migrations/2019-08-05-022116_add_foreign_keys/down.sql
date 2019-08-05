-- This file should undo anything in `up.sql`
ALTER TABLE tokens DROP CONSTRAINT user_id;
ALTER TABLE tasks DROP CONSTRAINT report_id;
ALTER TABLE tasks DROP CONSTRAINT profile_id;
ALTER TABLE reports DROP CONSTRAINT user_id;
ALTER TABLE worker_capabilities DROP CONSTRAINT worker_id;
ALTER TABLE worker_capabilities DROP CONSTRAINT profile_id;