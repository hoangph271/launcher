table! {
    auths (id) {
        id -> Text,
        auth_type -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        email -> Text,
        nickname -> Text,
    }
}

allow_tables_to_appear_in_same_query!(auths, users,);
