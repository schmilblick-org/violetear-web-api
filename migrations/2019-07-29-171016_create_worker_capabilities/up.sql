-- Your SQL goes here
CREATE TABLE worker_capabilities (
    id BIGSERIAL PRIMARY KEY,
    worker_id BIGSERIAL NOT NULL,
    kind TEXT NOT NULL,
    value TEXT NOT NULL
)