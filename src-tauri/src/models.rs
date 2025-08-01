use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Album {
    pub id: i32,
    pub jellyfin_id: String,
    pub title: String,
    pub artist: String,
    pub downloaded: bool,
    pub path: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::albums)]
pub struct NewAlbum {
    pub jellyfin_id: String,
    pub title: String,
    pub artist: String,
    pub downloaded: bool,
}
