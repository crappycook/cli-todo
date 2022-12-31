// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
    }
}
