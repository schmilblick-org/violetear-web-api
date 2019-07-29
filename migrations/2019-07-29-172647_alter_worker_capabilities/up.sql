-- Your SQL goes here
ALTER TABLE worker_capabilities DROP COLUMN kind;
ALTER TABLE worker_capabilities DROP COLUMN value;
ALTER TABLE worker_capabilities ADD COLUMN profile_id BIGSERIAL;