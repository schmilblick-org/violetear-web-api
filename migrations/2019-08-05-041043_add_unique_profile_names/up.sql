-- Your SQL goes here
ALTER TABLE profiles ADD CONSTRAINT name_keys UNIQUE (human_name, machine_name);