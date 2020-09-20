table! {
    access_types (value) {
        value -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    passwords (id) {
        id -> Uuid,
        user_id -> Uuid,
        digest -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_access_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        access_type -> Text,
        token -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_settings (id) {
        id -> Uuid,
        user_id -> Uuid,
        preferred_name -> Text,
        verified -> Bool,
        last_login_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(passwords -> users (user_id));
joinable!(user_access_tokens -> access_types (access_type));
joinable!(user_access_tokens -> users (user_id));
joinable!(user_settings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_types,
    passwords,
    user_access_tokens,
    user_settings,
    users,
);
