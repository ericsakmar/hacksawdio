// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        jellyfin_id -> Text,
        title -> Text,
        artist -> Text,
        path -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tracks (id) {
        id -> Integer,
        jellyfin_id -> Text,
        name -> Text,
        album_id -> Integer,
        path -> Nullable<Text>,
        track_index -> Nullable<Integer>,
    }
}

diesel::joinable!(tracks -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    tracks,
);
