// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> BigInt,
        title -> Text,
        content -> Text,
    }
}
