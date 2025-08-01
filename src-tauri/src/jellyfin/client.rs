use super::errors::JellyfinError;
use super::models::{
    AlbumSearchResponse, AlbumSearchResponseItem, AuthRequest, AuthResponse, JellyfinItem,
    JellyfinItemsResponse,
};
use crate::models::{Album, NewAlbum};
use crate::schema::albums::dsl::*;
use diesel::prelude::*;
use reqwest::{Client, StatusCode};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub struct JellyfinClient {
    base_url: String,
    http_client: Client,
    app_name: String,
    device_name: String,
    device_id: String,
    app_version: String,
    db_pool: crate::db::Pool,
}

impl JellyfinClient {
    pub fn new(
        base_url: String,
        app_name: String,
        device_name: String,
        device_id: String,
        app_version: String,
        db_pool: crate::db::Pool,
    ) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
            app_name,
            device_name,
            device_id,
            app_version,
            db_pool,
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
        // TODO there's got to be a way to chain these
        let items = self.search_jellyfin(search, access_token).await?;
        self.add_downloaded_state(&items).await
    }

    pub async fn download_album(
        &self,
        app_handle: &tauri::AppHandle,
        album_id: &str,
        access_token: &str,
    ) -> Result<(), JellyfinError> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        let album = self.sync_album(album_id, access_token).await?;

        if album.downloaded {
            return Ok(()); // already downloaded
        }

        let dir = self.create_album_dir(app_handle, &album.artist, &album.title)?;
        println!("Creating album directory: {:?}", dir);

        // mark it as downloaded
        diesel::update(albums.filter(jellyfin_id.eq(album_id)))
            .set(downloaded.eq(true))
            .execute(&mut conn)
            .map_err(|e| JellyfinError::DbError(e))?;

        Ok(())
    }

    pub async fn delete_album(&self, album_id: &str) -> Result<(), JellyfinError> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        // TODO actual delete

        diesel::update(albums.filter(jellyfin_id.eq(album_id)))
            .set(downloaded.eq(false))
            .execute(&mut conn)
            .map_err(|e| JellyfinError::DbError(e))?;

        Ok(())
    }

    async fn search_jellyfin(
        &self,
        search: &str,
        access_token: &str,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
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

    async fn get_jellyfin_item(
        &self,
        item_id: &str,
        access_token: &str,
    ) -> Result<JellyfinItem, JellyfinError> {
        let url = format!("{}/Items?ids={},recursive=true", self.base_url, item_id);

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
            let first = items.items.first().cloned();

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

    async fn add_downloaded_state(
        &self,
        res: &JellyfinItemsResponse,
    ) -> Result<AlbumSearchResponse, JellyfinError> {
        let album_ids = res
            .items
            .iter()
            .map(|item| item.id.clone())
            .collect::<Vec<_>>();

        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        let downloaded_albums: Vec<String> = albums
            .filter(jellyfin_id.eq_any(album_ids).and(downloaded.eq(true)))
            .select(jellyfin_id)
            .load(&mut conn)
            .map_err(|e| JellyfinError::DbError(e))?;

        let items = res
            .items
            .clone()
            .into_iter()
            .map(|item| AlbumSearchResponseItem {
                name: item.name,
                id: item.id.clone(),
                album_artist: item.album_artist,
                downloaded: downloaded_albums.contains(&item.id),
            })
            .collect::<Vec<_>>();

        Ok(AlbumSearchResponse {
            total_record_count: res.total_record_count,
            start_index: res.start_index,
            items,
        })
    }

    // GET TRACKS
    // http://hacksaw-house:8097/Items?parentId=53c1a2a3a8a4b8e1a69fc391081d198f&recursive=true&sortBy=IndexNumber

    async fn sync_album(&self, album_id: &str, access_token: &str) -> Result<Album, JellyfinError> {
        // consider passing this?
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        // Check if the album already exists in the database
        let local_album = albums
            .filter(jellyfin_id.eq(album_id))
            .first(&mut conn)
            .optional()
            .map_err(JellyfinError::DbError)?;

        match local_album {
            Some(album) => Ok(album),
            None => {
                // album does not exist, we will insert it
                let album_info = self.get_jellyfin_item(album_id, access_token).await?;

                let new_album = NewAlbum {
                    jellyfin_id: album_id.to_string(),
                    title: album_info.name,
                    artist: album_info.album_artist,
                    downloaded: false,
                };

                diesel::insert_into(albums)
                    .values(&new_album)
                    .execute(&mut conn)
                    .map_err(|e| JellyfinError::DbError(e))?;

                let new_local_album = albums
                    .filter(jellyfin_id.eq(album_id))
                    .first(&mut conn)
                    .optional()
                    .map_err(JellyfinError::DbError)?;

                match new_local_album {
                    Some(album) => Ok(album),
                    None => Err(JellyfinError::ApiError {
                        status: StatusCode::NOT_FOUND,
                        message: "Album not found after insertion".to_string(),
                    }),
                }
            }
        }
    }

    fn create_album_dir(
        &self,
        app_handle: &AppHandle,
        album_artist: &str,
        album_name: &str,
    ) -> Result<PathBuf, JellyfinError> {
        let mut app_data_path = app_handle.path().app_data_dir().map_err(|e| {
            JellyfinError::GenericError(format!("Failed to get app data dir: {}", e))
        })?;

        app_data_path.push("downloads");
        app_data_path.push(album_artist);
        app_data_path.push(album_name);

        if !app_data_path.exists() {
            fs::create_dir_all(&app_data_path).map_err(|e| {
                JellyfinError::GenericError(format!("Failed to create album directory: {}", e))
            })?;
        }

        Ok(app_data_path)
    }
}
