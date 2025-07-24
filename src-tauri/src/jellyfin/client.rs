use super::errors::JellyfinError;
use super::models::{AuthRequest, AuthResponse};
use reqwest::Client;

pub struct JellyfinClient {
    base_url: String,
    http_client: Client,
    app_name: String,
    device_name: String,
    device_id: String,
    app_version: String,
}

impl JellyfinClient {
    pub fn new(
        base_url: String,
        app_name: String,
        device_name: String,
        device_id: String,
        app_version: String,
    ) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
            app_name,
            device_name,
            device_id,
            app_version,
        }
    }

    // Helper to get common headers
    fn get_common_headers(&self) -> Vec<(&str, &str)> {
        vec![
            ("X-Emby-Client", &self.app_name),
            ("X-Emby-Device-Name", &self.device_name),
            ("X-Emby-Device-Id", &self.device_id),
            ("X-Emby-Client-Version", &self.app_version),
        ]
    }

    pub async fn authenticate_user_by_name(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, JellyfinError> {
        let auth_url = format!("{}/Users/AuthenticateByName", self.base_url);
        let request_body = AuthRequest {
            username: username.to_string(),
            pw: password.to_string(),
        };

        let mut request_builder = self.http_client.post(&auth_url);
        for (name, value) in self.get_common_headers() {
            request_builder = request_builder.header(name, value);
        }

        let response = request_builder.json(&request_body).send().await?;

        if response.status().is_success() {
            Ok(response.json::<AuthResponse>().await?)
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());
            Err(JellyfinError::ApiError {
                status,
                message: error_text,
            })
        }
    }
}
