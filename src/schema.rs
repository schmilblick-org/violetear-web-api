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

allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
