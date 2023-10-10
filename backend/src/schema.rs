// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        created -> Timestamp,
        active -> Bool,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        author -> Uuid,
        content -> Text,
        created -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 150]
        salt -> Nullable<Varchar>,
        #[max_length = 150]
        pass -> Varchar,
        created -> Timestamp,
        active -> Bool,
    }
}

diesel::joinable!(messages -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    messages,
    users,
);
