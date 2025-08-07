use super::errors::JellyfinError;
use super::models::{
    AlbumSearchResponse, AlbumSearchResponseItem, AuthRequest, AuthResponse, JellyfinItem,
    JellyfinItemsResponse,
};
use crate::jellyfin::models::{AlbumInfoResponse, AlbumTrackResponse};
use crate::models::{Album, NewAlbum};
use crate::schema::albums::dsl::*;
use diesel::prelude::*;
use futures::StreamExt;
use reqwest::{Client, StatusCode};
use sanitize_filename::sanitize;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use url::Url;

pub struct JellyfinClient {
    base_url: String,
    http_client: Client,
    app_name: String,
    device_name: String,
    device_id: String,
    app_version: String,
    db_pool: crate::db::Pool,
    download_queue: crate::download_queue::DownloadQueue,
}

impl JellyfinClient {
    pub fn new(
        base_url: String,
        app_name: String,
        device_name: String,
        device_id: String,
        app_version: String,
        db_pool: crate::db::Pool,
        download_queue: crate::download_queue::DownloadQueue,
    ) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
            app_name,
            device_name,
            device_id,
            app_version,
            db_pool,
            download_queue,
        }
    }

    // JELLYFIN STUFF
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
        limit: Option<u32>,
        offset: Option<u32>,
        user_id: Option<&str>,
    ) -> Result<AlbumSearchResponse, JellyfinError> {
        // return recents if search is empty
        if search.is_empty() {
            let recents = self
                .get_recents(access_token, limit, offset, user_id)
                .await?;

            return self.add_downloaded_state(&recents).await;
        }

        let album_results = self
            .search_jellyfin(
                Some(search),
                "MusicAlbum",
                access_token,
                None,
                None,
                None,
                None,
                user_id,
            )
            .await?;

        let artist_album_results = self
            .search_albums_by_artist(search, access_token, user_id)
            .await?;

        let mut combined_items =
            self.combine_jellyfin_items(album_results.items, artist_album_results.items);

        let total_record_count = combined_items.len() as u32;

        combined_items.sort_by(|a, b| a.name.cmp(&b.name));

        let paginated_items = combined_items
            .into_iter()
            .skip(offset.unwrap_or(0) as usize)
            .take(limit.unwrap_or(100) as usize)
            .collect::<Vec<_>>();

        let response: JellyfinItemsResponse = JellyfinItemsResponse {
            total_record_count,
            start_index: offset.unwrap_or(0),
            items: paginated_items,
        };

        self.add_downloaded_state(&response).await
    }

    // LOCAL ITEMS
    pub async fn search_albums_offline(
        &self,
        search: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<AlbumSearchResponse, JellyfinError> {
        if search.is_empty() {
            return self.get_recents_offline(limit, offset).await;
        }

        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        let local_albums = albums
            .filter(
                downloaded.eq(true).and(
                    title
                        .like(format!("%{}%", search))
                        .or(artist.like(format!("%{}%", search))),
                ),
            )
            .order(title.asc())
            .limit(limit.unwrap_or(100) as i64)
            .offset(offset.unwrap_or(0) as i64)
            .select(Album::as_select())
            .load::<Album>(&mut conn)
            .map_err(|e| JellyfinError::DbError(e))?;

        let items = local_albums
            .into_iter()
            .map(|album| AlbumSearchResponseItem {
                name: album.title,
                id: album.jellyfin_id.clone(),
                album_artist: album.artist,
                downloaded: album.downloaded,
            })
            .collect::<Vec<_>>();

        Ok(AlbumSearchResponse {
            total_record_count: items.len() as u32,
            start_index: offset.unwrap_or(0),
            items,
        })
    }

    pub async fn download_album(
        &self,
        app_handle: &tauri::AppHandle,
        album_id: &str,
        access_token: &str,
        user_id: Option<&str>,
    ) -> Result<(), JellyfinError> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        // get the album
        let album = self
            .sync_album(album_id, access_token, &mut conn, user_id)
            .await?;

        if album.downloaded {
            return Ok(());
        }

        // create the album directory
        let dir = self.create_album_dir(app_handle, &album.artist, &album.title)?;

        // get the tracks for the album
        let tracks = self.get_tracks(album_id, access_token).await?;
        let total_tracks = tracks.items.len();

        for track in tracks.items {
            let track_filename = self.generate_track_name(&track, total_tracks);
            let download_path = dir.join(&track_filename);

            diesel::insert_into(crate::schema::tracks::dsl::tracks)
                .values(crate::models::NewTrack {
                    jellyfin_id: &track.id,
                    name: &track.name,
                    album_id: album.id,
                    downloaded: false,
                    path: Some(download_path.to_string_lossy().to_string()),
                })
                .execute(&mut conn)
                .map_err(|e| JellyfinError::DbError(e))?;

            self.download_queue.add_track(
                crate::download_queue::Track {
                    track_id: track.id,
                    track_path: download_path.to_string_lossy().to_string(),
                },
                app_handle,
            );
        }

        // mark tracks as downloaded
        diesel::update(
            crate::schema::tracks::dsl::tracks
                .filter(crate::schema::tracks::dsl::album_id.eq(album.id)),
        )
        .set(crate::schema::tracks::dsl::downloaded.eq(true))
        .execute(&mut conn)
        .map_err(|e| JellyfinError::DbError(e))?;

        // mark album as downloaded
        diesel::update(albums.filter(jellyfin_id.eq(album_id)))
            .set((
                downloaded.eq(true),
                path.eq(dir.to_string_lossy().to_string()),
                updated_at.eq(diesel::dsl::now),
            ))
            .execute(&mut conn)
            .map_err(|e| JellyfinError::DbError(e))?;

        Ok(())
    }

    pub async fn delete_album(&self, album_id: &str) -> Result<(), JellyfinError> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        let album = self.find_album(album_id, &mut conn)?;

        match album {
            Some(album) => {
                if let Some(album_path) = &album.path {
                    let path_buf = PathBuf::from(album_path);
                    if path_buf.exists() {
                        fs::remove_dir_all(&path_buf).map_err(|e| {
                            JellyfinError::GenericError(format!(
                                "Failed to delete album dir: {}",
                                e
                            ))
                        })?;
                    }

                    // remove the artis directory if it's now empty
                    let parent_dir = path_buf.parent().ok_or_else(|| {
                        JellyfinError::GenericError("Failed to get parent directory".to_string())
                    })?;

                    if parent_dir.exists() && parent_dir.is_dir() {
                        let entries = fs::read_dir(parent_dir).map_err(|e| {
                            JellyfinError::GenericError(format!("Failed to read dir: {}", e))
                        })?;
                        if entries.count() == 0 {
                            fs::remove_dir(parent_dir).map_err(|e| {
                                JellyfinError::GenericError(format!(
                                    "Failed to delete parent dir: {}",
                                    e
                                ))
                            })?;
                        }
                    }
                }

                // mark tracks as not downloaded
                diesel::update(
                    crate::schema::tracks::dsl::tracks
                        .filter(crate::schema::tracks::dsl::album_id.eq(album.id)),
                )
                .set(crate::schema::tracks::dsl::downloaded.eq(false))
                .execute(&mut conn)
                .map_err(|e| JellyfinError::DbError(e))?;

                // mark album as not downloaded
                diesel::update(albums.filter(jellyfin_id.eq(album_id)))
                    .set((downloaded.eq(false), path.eq(None::<String>)))
                    .execute(&mut conn)
                    .map_err(|e| JellyfinError::DbError(e))?;

                Ok(())
            }
            None => {
                return Err(JellyfinError::ApiError {
                    status: StatusCode::NOT_FOUND,
                    message: "Album not found".to_string(),
                })
            }
        }
    }

    async fn search_albums_by_artist(
        &self,
        search: &str,
        access_token: &str,
        user_id: Option<&str>,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
        let artist_results = self
            .search_jellyfin(
                Some(search),
                "MusicArtist",
                access_token,
                None,
                None,
                None,
                None,
                user_id,
            )
            .await?;

        let artist_ids: Vec<String> = artist_results
            .items
            .iter()
            .map(|item| item.id.clone())
            .collect();

        // exit early if no artist IDs are found
        if artist_ids.is_empty() {
            return Ok(JellyfinItemsResponse {
                total_record_count: 0,
                start_index: 0,
                items: Vec::new(),
            });
        }

        let artist_album_results = self
            .search_jellyfin(
                None,
                "MusicAlbum",
                access_token,
                None,
                None,
                None,
                Some(artist_ids),
                user_id,
            )
            .await;

        artist_album_results
    }

    async fn search_jellyfin(
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

    async fn get_jellyfin_item(
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
                album_artist: item
                    .album_artist
                    .unwrap_or_else(|| "Unknown Artist".to_string()),
                downloaded: downloaded_albums.contains(&item.id),
            })
            .collect::<Vec<_>>();

        Ok(AlbumSearchResponse {
            total_record_count: res.total_record_count,
            start_index: res.start_index,
            items,
        })
    }

    async fn get_tracks(
        &self,
        album_id: &str,
        access_token: &str,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
        // http://hacksaw-house:8097/Items?parentId=53c1a2a3a8a4b8e1a69fc391081d198f&recursive=true&sortBy=IndexNumber
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

        let mut dest_file = File::create(&download_path)
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

    pub async fn get_album_info(&self, album_id: &str) -> Result<AlbumInfoResponse, JellyfinError> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        let local_album_res = self.find_album(album_id, &mut conn)?;

        match local_album_res {
            Some(local_album) => {
                // get the tracks for the album
                let local_tracks = crate::schema::tracks::dsl::tracks
                    .filter(crate::schema::tracks::dsl::album_id.eq(local_album.id))
                    .select(crate::models::Track::as_select())
                    .load::<crate::models::Track>(&mut conn)
                    .map_err(|e| JellyfinError::DbError(e))?;

                let result = AlbumInfoResponse {
                    name: local_album.title,
                    artist: local_album.artist,
                    tracks: local_tracks
                        .into_iter()
                        .map(|track| AlbumTrackResponse {
                            name: track.name,
                            playback_url: track.path.unwrap_or_default(),
                        })
                        .collect(),
                };

                Ok(result)
            }
            None => Err(JellyfinError::ApiError {
                status: StatusCode::NOT_FOUND,
                message: "Album not found".to_string(),
            }),
        }
    }

    async fn sync_album(
        &self,
        album_id: &str,
        access_token: &str,
        conn: &mut crate::db::Connection,
        user_id: Option<&str>,
    ) -> Result<Album, JellyfinError> {
        // Check if the album already exists in the database
        let local_album = self.find_album(album_id, conn)?;

        match local_album {
            Some(album) => Ok(album),
            None => {
                // album does not exist, we will insert it
                let album_info = self
                    .get_jellyfin_item(album_id, access_token, user_id)
                    .await?;

                let new_album = NewAlbum {
                    jellyfin_id: album_id,
                    title: &album_info.name,
                    artist: &album_info
                        .album_artist
                        .unwrap_or_else(|| "Unknown Artist".to_string()),
                    downloaded: false,
                };

                diesel::insert_into(albums)
                    .values(&new_album)
                    .execute(conn)
                    .map_err(|e| JellyfinError::DbError(e))?;

                let new_local_album = self.find_album(album_id, conn)?;

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

    fn find_album(
        &self,
        album_id: &str,
        conn: &mut crate::db::Connection,
    ) -> Result<Option<Album>, JellyfinError> {
        albums
            .filter(jellyfin_id.eq(album_id))
            .select(Album::as_select())
            .first(conn)
            .optional()
            .map_err(JellyfinError::DbError)
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
        app_data_path.push(sanitize(album_artist));
        app_data_path.push(sanitize(album_name));

        if !app_data_path.exists() {
            fs::create_dir_all(&app_data_path).map_err(|e| {
                JellyfinError::GenericError(format!("Failed to create album directory: {}", e))
            })?;
        }

        Ok(app_data_path)
    }

    fn generate_track_name(&self, track: &JellyfinItem, total_tracks: usize) -> String {
        let extension = match track.container.as_ref() {
            Some(ext) => format!(".{}", ext),
            None => "".to_string(),
        };

        let width = if total_tracks == 0 {
            2
        } else {
            total_tracks.to_string().len()
        };

        let track_number = format!("{:0width$}", track.index_number.unwrap_or(0), width = width);

        format!("{} - {}{}", track_number, sanitize(&track.name), extension)
    }

    fn combine_jellyfin_items(
        &self,
        list1: Vec<JellyfinItem>,
        list2: Vec<JellyfinItem>,
    ) -> Vec<JellyfinItem> {
        let mut unique_items: HashSet<JellyfinItem> = HashSet::new();

        for item in list1 {
            unique_items.insert(item);
        }

        for item in list2 {
            unique_items.insert(item);
        }

        unique_items.into_iter().collect()
    }

    async fn get_recents(
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

    async fn get_recents_offline(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<AlbumSearchResponse, JellyfinError> {
        let mut conn = self
            .db_pool
            .get()
            .map_err(|e| JellyfinError::DbPoolError(e))?;

        let local_albums = albums
            .filter(downloaded.eq(true))
            .order(updated_at.desc())
            .limit(limit.unwrap_or(100) as i64)
            .offset(offset.unwrap_or(0) as i64)
            .select(Album::as_select())
            .load::<Album>(&mut conn)
            .map_err(|e| JellyfinError::DbError(e))?;

        let items = local_albums
            .into_iter()
            .map(|album| AlbumSearchResponseItem {
                name: album.title,
                id: album.jellyfin_id.clone(),
                album_artist: album.artist,
                downloaded: album.downloaded,
            })
            .collect::<Vec<_>>();

        Ok(AlbumSearchResponse {
            total_record_count: items.len() as u32,
            start_index: offset.unwrap_or(0),
            items,
        })
    }
}
