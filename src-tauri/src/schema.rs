// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        jellyfin_id -> Text,
        title -> Text,
        artist -> Text,
        downloaded -> Bool,
    }
}
