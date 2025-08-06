// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        jellyfin_id -> Text,
        title -> Text,
        artist -> Text,
        downloaded -> Bool,
        path -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tracks (id) {
        id -> Integer,
        jellyfin_id -> Text,
        name -> Text,
        album_id -> Integer,
        path -> Nullable<Text>,
        downloaded -> Bool,
    }
}

diesel::joinable!(tracks -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    tracks,
);
