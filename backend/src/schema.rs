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
    message_recipients (id) {
        id -> Uuid,
        message_id -> Uuid,
        recipient -> Nullable<Uuid>,
        recipient_group -> Nullable<Uuid>,
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

diesel::joinable!(message_recipients -> groups (recipient_group));
diesel::joinable!(message_recipients -> messages (message_id));
diesel::joinable!(message_recipients -> users (recipient));
diesel::joinable!(messages -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    message_recipients,
    messages,
    users,
);
