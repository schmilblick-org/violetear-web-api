table! {
    profiles (id) {
        id -> Int8,
        machine_name -> Text,
        human_name -> Text,
        module -> Text,
        config -> Nullable<Jsonb>,
    }
}

table! {
    reports (id) {
        id -> Int8,
        user_id -> Int8,
        created_when -> Timestamptz,
        file_multihash -> Text,
        file -> Nullable<Bytea>,
    }
}

table! {
    tasks (id) {
        id -> Int8,
        report_id -> Int8,
        profile_id -> Int8,
        created_when -> Timestamptz,
        completed_when -> Nullable<Timestamptz>,
        status -> Text,
    }
}

table! {
    tokens (id) {
        id -> Int8,
        user_id -> Int8,
        token -> Text,
        created_when -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Text,
        hashed_password -> Text,
        rank -> Int4,
    }
}

table! {
    worker_capabilities (id) {
        id -> Int8,
        worker_id -> Int8,
        profile_id -> Int8,
    }
}

table! {
    workers (id) {
        id -> Int8,
        last_active -> Timestamptz,
    }
}

joinable!(tasks -> reports (report_id));
joinable!(worker_capabilities -> workers (worker_id));

allow_tables_to_appear_in_same_query!(
    profiles,
    reports,
    tasks,
    tokens,
    users,
    worker_capabilities,
    workers,
);
