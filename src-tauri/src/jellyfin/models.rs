use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthRequest {
    pub username: String,
    pub pw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub server_id: String,
    pub user: UserDetails,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct UserDetails {
    pub id: String,
    pub name: String,
    pub primary_image_tag: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SessionResponse {
    pub authenticated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct JellyfinItemsResponse {
    pub total_record_count: u32,
    pub start_index: u32,
    pub items: Vec<JellyfinItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct JellyfinItem {
    pub name: String,
    pub id: String,
    pub album_artist: Option<String>,
    pub container: Option<String>,
    pub index_number: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct AlbumSearchResponse {
    pub total_record_count: u32,
    pub start_index: u32,
    pub items: Vec<AlbumSearchResponseItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct AlbumSearchResponseItem {
    pub name: String,
    pub id: String,
    pub album_artist: String,
    pub downloaded: bool,
}
