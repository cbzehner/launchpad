table! {
    access (id) {
        id -> Uuid,
        user_id -> Uuid,
        access_type -> Text,
        access_digest -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    access_types (value) {
        value -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        preferred_name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(access -> access_types (access_type));
joinable!(access -> users (user_id));

allow_tables_to_appear_in_same_query!(access, access_types, users,);
