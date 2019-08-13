-- Your SQL goes here
CREATE TABLE profiles (
    id BIGSERIAL PRIMARY KEY,
    machine_name TEXT NOT NULL,
    human_name TEXT NOT NULL,
    module TEXT NOT NULL,
    config JSONB
)