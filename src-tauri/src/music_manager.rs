use crate::jellyfin::client::JellyfinClient;
use crate::jellyfin::errors::JellyfinError;
use crate::jellyfin::models::{
    AlbumInfoResponse, AlbumSearchResponse, AlbumSearchResponseItem, AlbumTrackResponse,
    AuthResponse, JellyfinItem, JellyfinItemsResponse,
};
use crate::models::Album;
use crate::repository::Repository;
use reqwest::StatusCode;
use sanitize_filename::sanitize;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub struct MusicManager {
    jellyfin_client: JellyfinClient,
    pub repository: Repository,
    download_queue: crate::download_queue::DownloadQueue,
}

impl MusicManager {
    pub fn new(
        jellyfin_client: JellyfinClient,
        repository: Repository,
        download_queue: crate::download_queue::DownloadQueue,
    ) -> Self {
        Self {
            jellyfin_client,
            repository,
            download_queue,
        }
    }

    pub async fn authenticate_user_by_name(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, JellyfinError> {
        self.jellyfin_client
            .authenticate_user_by_name(username, password)
            .await
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
                .jellyfin_client
                .get_recents(access_token, limit, offset, user_id)
                .await?;

            return self.add_downloaded_state(&recents).await;
        }

        let album_results = self
            .jellyfin_client
            .search_albums(search, access_token, user_id)
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

    pub async fn search_albums_offline(
        &self,
        search: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<AlbumSearchResponse, JellyfinError> {
        if search.is_empty() {
            return self.get_recents_offline(limit, offset).await;
        }

        let local_albums = self
            .repository
            .search_albums_offline(search, limit, offset)
            .map_err(|e| JellyfinError::GenericError(e.to_string()))?;

        let items = local_albums
            .into_iter()
            .map(|album| AlbumSearchResponseItem {
                name: album.title,
                id: album.jellyfin_id.clone(),
                album_artist: album.artist,
                downloaded: album.path.is_some(),
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
        user_id: &str,
    ) -> Result<(), JellyfinError> {
        self.download_queue.add_album(
            crate::download_queue::Album {
                album_id: album_id.to_string(),
                user_id: user_id.to_string(),
            },
            app_handle,
        );
        Ok(())
    }

    pub async fn delete_album(&self, album_id: &str) -> Result<(), JellyfinError> {
        let album = self
            .repository
            .find_album(album_id)
            .map_err(|e| JellyfinError::GenericError(e.to_string()))?
            .ok_or_else(|| JellyfinError::ApiError {
                status: StatusCode::NOT_FOUND,
                message: "Album not found".to_string(),
            })?;

        if let Some(album_path) = &album.path {
            let path_buf = PathBuf::from(album_path);
            if path_buf.exists() {
                fs::remove_dir_all(&path_buf).map_err(|e| {
                    JellyfinError::GenericError(format!("Failed to delete album dir: {}", e))
                })?;
            }

            // remove the artist directory if it's now empty
            let parent_dir = path_buf.parent().ok_or_else(|| {
                JellyfinError::GenericError("Failed to get parent directory".to_string())
            })?;

            if parent_dir.exists() && parent_dir.is_dir() {
                let entries = fs::read_dir(parent_dir).map_err(|e| {
                    JellyfinError::GenericError(format!("Failed to read dir: {}", e))
                })?;

                if entries.count() == 0 {
                    fs::remove_dir(parent_dir).map_err(|e| {
                        JellyfinError::GenericError(format!("Failed to delete parent dir: {}", e))
                    })?;
                }
            }
        }

        self.repository
            .delete_album_and_tracks(&album)
            .map_err(|e| JellyfinError::GenericError(e.to_string()))?;

        Ok(())
    }

    async fn search_albums_by_artist(
        &self,
        search: &str,
        access_token: &str,
        user_id: Option<&str>,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
        let artist_results = self
            .jellyfin_client
            .search_artists(search, access_token, user_id)
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
            .jellyfin_client
            .search_albums_by_artist(artist_ids, access_token, user_id)
            .await;

        artist_album_results
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

        let downloaded_albums = self
            .repository
            .get_downloaded_album_ids(album_ids)
            .map_err(|e| JellyfinError::GenericError(e.to_string()))?;

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

    pub async fn download_track(
        &self,
        track_id: &str,
        download_path: &str,
        access_token: &str,
    ) -> Result<(), JellyfinError> {
        self.jellyfin_client
            .download_track(track_id, download_path, access_token)
            .await
    }

    pub async fn download_album_art(
        &self,
        album_id: &str,
        image_id: &str,
        download_path: &str,
        access_token: &str,
    ) -> Result<(), JellyfinError> {
        self.jellyfin_client
            .download_album_art(album_id, image_id, download_path, access_token)
            .await
    }

    pub async fn get_tracks(
        &self,
        album_id: &str,
        access_token: &str,
    ) -> Result<JellyfinItemsResponse, JellyfinError> {
        self.jellyfin_client
            .get_tracks(album_id, access_token)
            .await
    }

    pub async fn get_album_info(&self, album_id: &str) -> Result<AlbumInfoResponse, JellyfinError> {
        let (local_album, local_tracks) = self
            .repository
            .get_album_details(album_id)
            .map_err(|e| JellyfinError::GenericError(e.to_string()))?
            .ok_or_else(|| JellyfinError::ApiError {
                status: StatusCode::NOT_FOUND,
                message: "Album not found".to_string(),
            })?;

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
            image_url: local_album.image_path,
        };

        Ok(result)
    }

    pub async fn sync_album(
        &self,
        album_id: &str,
        access_token: &str,
        user_id: Option<&str>,
    ) -> Result<Album, JellyfinError> {
        // Check if the album already exists in the database
        if let Some(album) = self
            .repository
            .find_album(album_id)
            .map_err(|e| JellyfinError::GenericError(e.to_string()))?
        {
            return Ok(album);
        }

        // album does not exist, we will insert it
        let album_info = self
            .jellyfin_client
            .get_jellyfin_item(album_id, access_token, user_id)
            .await?;

        let image_id = album_info
            .image_tags
            .as_ref()
            .and_then(|tags| tags.primary.as_deref());

        self.repository
            .create_album(
                album_id,
                &album_info.name,
                &album_info
                    .album_artist
                    .unwrap_or_else(|| "Unknown Artist".to_string()),
                image_id,
            )
            .map_err(|e| JellyfinError::GenericError(e.to_string()))
    }

    pub fn create_album_dir(
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

    pub fn generate_track_name(&self, track: &JellyfinItem, total_tracks: usize) -> String {
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

    async fn get_recents_offline(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<AlbumSearchResponse, JellyfinError> {
        let local_albums = self
            .repository
            .get_recents_offline(limit, offset)
            .map_err(|e| JellyfinError::GenericError(e.to_string()))?;

        let items = local_albums
            .into_iter()
            .map(|album| AlbumSearchResponseItem {
                name: album.title,
                id: album.jellyfin_id.clone(),
                album_artist: album.artist,
                downloaded: album.path.is_some(),
            })
            .collect::<Vec<_>>();

        Ok(AlbumSearchResponse {
            total_record_count: items.len() as u32,
            start_index: offset.unwrap_or(0),
            items,
        })
    }
}
