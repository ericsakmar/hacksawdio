use crate::schema::albums;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub external_id: String,
    pub name: String,
    pub artist: String,
    pub downloaded: bool,
}

#[derive(Insertable)]
#[diesel(table_name = albums)]
pub struct NewAlbum<'a> {
    pub external_id: &'a str,
    pub name: &'a str,
    pub artist: &'a str,
}

