// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        login -> Text,
        email -> Text,
        password_hash -> Text,
    }
}
