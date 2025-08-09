use crate::jellyfin::errors::JellyfinError;
use crate::jellyfin::models::{AuthRequest, AuthResponse, JellyfinItem, JellyfinItemsResponse};
use futures::StreamExt;
use reqwest::{Client, StatusCode};
use tokio::io::AsyncWriteExt;
use url::Url;

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

    pub async fn search_jellyfin(
        &self,
        search: Option<&str>,
        item_types: &str,
        access_token: &str,
        sort_by: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
        artist_ids: Option<Vec<String>>,
        user_id: Option<&str>,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let mut url = Url::parse(&self.base_url)
            .map_err(|e| JellyfinError::GenericError(format!("Invalid base URL: {}", e)))?;

        if let Some(user_id) = user_id {
            url.set_path(&format!("/Users/{}/Items", user_id));
        } else {
            url.set_path("/Items");
        }

        url.query_pairs_mut()
            .append_pair("includeItemTypes", item_types)
            .append_pair("recursive", "true")
            .append_pair("limit", &limit.to_string())
            .append_pair("startIndex", &offset.to_string())
            .append_pair("sortBy", "Album,AlbumArtist");

        if let Some(search_term) = search {
            url.query_pairs_mut().append_pair("searchTerm", search_term);
        }

        if let Some(artist_ids) = artist_ids {
            let artist_ids_str = artist_ids.join(",");
            url.query_pairs_mut()
                .append_pair("artistIds", &artist_ids_str);
        }

        if let Some(sort_by) = sort_by {
            url.query_pairs_mut().append_pair("sortBy", sort_by);
        }

        let response = self
            .http_client
            .get(&url.to_string())
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
            let items = response.json::<JellyfinItemsResponse>().await?;
            Ok(items)
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

    pub async fn get_jellyfin_item(
        &self,
        item_id: &str,
        access_token: &str,
        user_id: Option<&str>,
    ) -> Result<JellyfinItem, JellyfinError> {
        let mut url = Url::parse(&self.base_url)
            .map_err(|e| JellyfinError::GenericError(format!("Invalid base URL: {}", e)))?;

        if let Some(user_id) = user_id {
            url.set_path(&format!("/Users/{}/Items", user_id));
        } else {
            url.set_path("/Items");
        }

        url.query_pairs_mut()
            .append_pair("ids", item_id)
            .append_pair("recursive", "true");

        let response = self
            .http_client
            .get(url.to_string())
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
            let mut items = response.json::<JellyfinItemsResponse>().await?;
            let first = items.items.drain(..).next();

            if let Some(item) = first {
                Ok(item)
            } else {
                Err(JellyfinError::ApiError {
                    status: StatusCode::NOT_FOUND,
                    message: "Item not found".to_string(),
                })
            }
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

    pub async fn get_tracks(
        &self,
        album_id: &str,
        access_token: &str,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
        let url = format!(
            "{}/Items?parentId={}&recursive=true&sortBy=IndexNumber",
            self.base_url, album_id
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
            let items = response.json::<JellyfinItemsResponse>().await?;
            Ok(items)
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

    pub async fn download_track(
        &self,
        track_id: &str,
        download_path: &str,
        access_token: &str,
    ) -> Result<(), JellyfinError> {
        let url = format!("{}/Items/{}/Download", self.base_url, track_id);

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

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "N/A".to_string());
            return Err(JellyfinError::ApiError {
                status,
                message: body,
            });
        }

        let mut dest_file = tokio::fs::File::create(&download_path)
            .await
            .map_err(|e| JellyfinError::GenericError(format!("Failed to create file: {}", e)))?;

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            dest_file.write_all(&chunk).await.map_err(|e| {
                JellyfinError::GenericError(format!("Failed to write chunk: {}", e))
            })?;
        }

        dest_file
            .flush()
            .await
            .map_err(|e| JellyfinError::GenericError(format!("Failed to flush file: {}", e)))?;

        Ok(())
    }

    pub async fn get_recents(
        &self,
        access_token: &str,
        limit: Option<u32>,
        offset: Option<u32>,
        user_id: Option<&str>,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let mut url = Url::parse(&self.base_url)
            .map_err(|e| JellyfinError::GenericError(format!("Invalid base URL: {}", e)))?;

        if let Some(user_id) = user_id {
            url.set_path(&format!("/Users/{}/Items/Latest", user_id));
        } else {
            url.set_path("/Items/Latest");
        }

        url.query_pairs_mut()
            .append_pair("includeItemTypes", "MusicAlbum")
            .append_pair("limit", &limit.to_string())
            .append_pair("startIndex", &offset.to_string());

        let response = self
            .http_client
            .get(&url.to_string())
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
            let items = response.json::<Vec<JellyfinItem>>().await?;

            Ok(JellyfinItemsResponse {
                total_record_count: items.len() as u32,
                start_index: offset,
                items,
            })
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
