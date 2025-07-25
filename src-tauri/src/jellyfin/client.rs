use super::errors::JellyfinError;
use super::models::{
    AlbumSearchResponse, AlbumSearchResponseItem, AuthRequest, AuthResponse, JellyfinItemsResponse,
};
use crate::schema::albums::dsl::*;
use diesel::prelude::*;
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

        let response = self
            .http_client
            .post(&auth_url)
            .header(
                "Authorization",
                format!(
                    "MediaBrowser Client=\"{}\", Device=\"{}\", DeviceId=\"{}\", Version=\"{}\"",
                    self.app_name, self.device_name, self.device_id, self.app_version
                ),
            )
            .json(&request_body)
            .send()
            .await?;

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

    pub async fn search_albums(
        &self,
        search: &str,
        access_token: &str,
    ) -> Result<AlbumSearchResponse, JellyfinError> {
        let url = format!(
            "{}/Items?includeItemTypes=MusicAlbum&searchTerm={}&recursive=true&limit=100",
            self.base_url, search
        );

        let response = self
            .http_client
            .get(&url)
            .header(
                "Authorization",
                format!(
                    "MediaBrowser Token=\"{}\", Client=\"{}\", Device=\"{}\", DeviceId=\"{}\", Version=\"{}\"",
                    access_token, self.app_name, self.device_name, self.device_id, self.app_version
                ),
            )
            .send()
            .await?;

        if response.status().is_success() {
            // TODO map one into the other?
            let items = response.json::<JellyfinItemsResponse>().await?;
            let with_downloaded_state = self.add_downloaded_state(&items).await;
            Ok(with_downloaded_state)
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

    pub async fn download_album(
        &self,
        album_id: &str,
        access_token: &str,
        db_pool: &crate::db::Pool,
    ) -> Result<(), JellyfinError> {
        let mut conn = db_pool.get().map_err(|e| JellyfinError::DbPoolError(e))?;

        diesel::insert_into(albums)
            .values((
                jellyfin_id.eq(album_id),
                title.eq("fetch from jellyfin"),
                artist.eq("fetch from jellyfin"),
                downloaded.eq(true),
            ))
            .on_conflict(jellyfin_id)
            .do_update()
            .set(downloaded.eq(true))
            .execute(&mut conn)
            .map_err(|e| JellyfinError::DbError(e))?;

        Ok(())
    }

    async fn add_downloaded_state(&self, res: &JellyfinItemsResponse) -> AlbumSearchResponse {
        let items = res
            .items
            .clone()
            .into_iter()
            .map(|item| {
                AlbumSearchResponseItem {
                    name: item.name,
                    id: item.id,
                    album_artist: item.album_artist,
                    downloaded: false, // Default to false, will be updated later
                }
            })
            .collect::<Vec<_>>();

        AlbumSearchResponse {
            total_record_count: res.total_record_count,
            start_index: res.start_index,
            items,
        }
    }
}
