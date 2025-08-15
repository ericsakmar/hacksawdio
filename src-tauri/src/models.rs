use crate::schema::{albums, tracks};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Album {
    pub id: i32,
    pub jellyfin_id: String,
    pub title: String,
    pub artist: String,
    pub path: Option<String>,
    pub image_id: Option<String>,
    pub image_path: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = albums)]
pub struct NewAlbum<'a> {
    pub jellyfin_id: &'a str,
    pub title: &'a str,
    pub artist: &'a str,
    pub image_id: Option<&'a str>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Album))]
#[diesel(table_name = tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Track {
    pub id: i32,
    pub jellyfin_id: String,
    pub name: String,
    pub album_id: i32,
    pub path: Option<String>,
    pub track_index: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = tracks)]
pub struct NewTrack<'a> {
    pub jellyfin_id: &'a str,
    pub name: &'a str,
    pub album_id: i32,
    pub path: Option<String>,
    pub track_index: i32,
}
