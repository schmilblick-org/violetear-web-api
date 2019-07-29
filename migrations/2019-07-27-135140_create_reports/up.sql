-- Your SQL goes here
CREATE TABLE reports (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    created_when TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    file_multihash TEXT NOT NULL,
    file BYTEA
)