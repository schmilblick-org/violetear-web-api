-- Your SQL goes here
ALTER TABLE tokens ADD CONSTRAINT user_id_foreign FOREIGN KEY (user_id) REFERENCES users(id);
ALTER TABLE tasks ADD CONSTRAINT report_id_foreign FOREIGN KEY (report_id) REFERENCES reports(id);
ALTER TABLE tasks ADD CONSTRAINT profile_id_foreign FOREIGN KEY (profile_id) REFERENCES profiles(id);
ALTER TABLE reports ADD CONSTRAINT user_id_foreign FOREIGN KEY (user_id) REFERENCES users(id);
ALTER TABLE worker_capabilities ADD CONSTRAINT worker_id_foreign FOREIGN KEY (worker_id) REFERENCES workers(id);
ALTER TABLE worker_capabilities ADD CONSTRAINT profile_id_foreign FOREIGN KEY (profile_id) REFERENCES profiles(id);