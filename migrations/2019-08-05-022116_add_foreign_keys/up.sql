-- Your SQL goes here
ALTER TABLE tokens ADD CONSTRAINT user_id FOREIGN KEY (id) REFERENCES users(id);
ALTER TABLE tasks ADD CONSTRAINT report_id FOREIGN KEY (id) REFERENCES reports(id);
ALTER TABLE tasks ADD CONSTRAINT profile_id FOREIGN KEY (id) REFERENCES profiles(id);
ALTER TABLE reports ADD CONSTRAINT user_id FOREIGN KEY (id) REFERENCES users(id);
ALTER TABLE worker_capabilities ADD CONSTRAINT worker_id FOREIGN KEY (id) REFERENCES workers(id);
ALTER TABLE worker_capabilities ADD CONSTRAINT profile_id FOREIGN KEY (id) REFERENCES profiles(id);