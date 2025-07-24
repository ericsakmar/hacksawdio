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
