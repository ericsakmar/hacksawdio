use crate::jellyfin::errors::JellyfinError;
use crate::models::NewTrack;
use crate::music_manager::MusicManager;
use std::sync::{mpsc, Arc, Mutex};
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
struct DownloadQueueEmpty;

#[derive(Clone, serde::Serialize)]
struct DownloadQueueNotEmpty;

#[derive(Clone, serde::Serialize)]
struct AlbumDownloadStarted {
    album_id: String,
}

#[derive(Clone, serde::Serialize)]
struct AlbumDownloadCompleted {
    album_id: String,
}

// Track details for a download
pub struct Album {
    pub album_id: String,
    pub user_id: String,
}

// Message type for the download queue channel
pub enum DownloadQueueMessage {
    NewAlbum(Album),
    Shutdown,
}

#[derive(Clone)]
pub struct DownloadQueue {
    sender: mpsc::Sender<DownloadQueueMessage>,
}

impl DownloadQueue {
    // Create a new queue and return it along with the receiver end of the channel
    pub fn new() -> (Self, mpsc::Receiver<DownloadQueueMessage>) {
        let (sender, receiver) = mpsc::channel();
        (Self { sender }, receiver)
    }

    pub fn add_album(&self, album: Album, app_handle: &AppHandle) {
        app_handle
            .emit("download-queue-not-empty", DownloadQueueNotEmpty)
            .unwrap();

        self.sender
            .send(DownloadQueueMessage::NewAlbum(album))
            .unwrap();
    }

    // Method to send a shutdown signal
    pub fn shutdown(&self) {
        self.sender.send(DownloadQueueMessage::Shutdown).unwrap();
    }
}

fn handle_message(
    message: DownloadQueueMessage,
    app_handle: &AppHandle,
    rt: &tokio::runtime::Runtime,
    music_manager: &Arc<MusicManager>,
    auth_token: &Arc<Mutex<Option<String>>>,
) -> bool {
    // returns false if shutdown
    match message {
        DownloadQueueMessage::NewAlbum(album) => {
            let token = {
                let token_guard = auth_token.lock().unwrap();
                token_guard.clone()
            };

            if let Some(token) = token {
                app_handle
                    .emit(
                        "album-download-started",
                        AlbumDownloadStarted {
                            album_id: album.album_id.clone(),
                        },
                    )
                    .unwrap();

                rt.block_on(async {
                    let album_download_result: Result<(), JellyfinError> = async {
                        let local_album = music_manager
                            .sync_album(&album.album_id, &token, Some(&album.user_id))
                            .await?;

                        // already downloaded
                        if local_album.path.is_some() {
                            return Ok(());
                        }

                        // create the album directory
                        let dir = music_manager.create_album_dir(
                            app_handle,
                            &local_album.artist,
                            &local_album.title,
                        )?;

                        let image_path = local_album
                            .image_id
                            .as_ref()
                            .map(|_| dir.join("cover.jpg").to_string_lossy().to_string());

                        // get the album art if we have it
                        if let Some(image_path) = &image_path {
                            music_manager
                                .download_album_art(
                                    &local_album.jellyfin_id,
                                    &local_album.image_id.as_ref().unwrap(),
                                    &image_path,
                                    &token,
                                )
                                .await?;
                        }

                        // get the tracks for the album
                        let tracks = music_manager.get_tracks(&album.album_id, &token).await?;
                        let total_tracks = tracks.items.len();

                        for track in tracks.items {
                            let track_filename =
                                music_manager.generate_track_name(&track, total_tracks);
                            let download_path = dir.join(&track_filename);

                            music_manager
                                .repository
                                .insert_track(&NewTrack {
                                    jellyfin_id: &track.id,
                                    name: &track.name,
                                    album_id: local_album.id,
                                    path: Some(download_path.to_string_lossy().to_string()),
                                    track_index: track.index_number.unwrap_or(0) as i32,
                                })
                                .map_err(|e| JellyfinError::GenericError(e.to_string()))?;

                            music_manager
                                .download_track(&track.id, &download_path.to_string_lossy(), &token)
                                .await?;
                        }

                        // mark album as downloaded
                        music_manager
                            .repository
                            .mark_album_as_downloaded(
                                &album.album_id,
                                &dir.to_string_lossy(),
                                image_path.as_deref(),
                            )
                            .map_err(|e| JellyfinError::GenericError(e.to_string()))
                    }
                    .await;

                    if let Err(e) = album_download_result {
                        let error_message = e.to_string();
                        eprintln!(
                            "Error downloading album {}: {}",
                            &album.album_id, &error_message
                        );
                    } else {
                        app_handle
                            .emit(
                                "album-download-completed",
                                AlbumDownloadCompleted {
                                    album_id: album.album_id.clone(),
                                },
                            )
                            .unwrap();
                    }
                });
            } else {
                let error_message = "Download failed: No auth token available.".to_string();
                eprintln!("{}", &error_message);
            }
            true
        }
        DownloadQueueMessage::Shutdown => {
            println!("Download queue shutting down.");
            false
        }
    }
}

// The processor function, to be run in a thread
pub fn process_downloads(
    app_handle: AppHandle,
    receiver: mpsc::Receiver<DownloadQueueMessage>,
    jellyfin_client: Arc<MusicManager>,
    auth_token: Arc<Mutex<Option<String>>>,
) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    'main_loop: loop {
        // Wait for the first message of a batch
        let mut current_message = match receiver.recv() {
            Ok(msg) => msg,
            Err(_) => {
                println!("Download queue channel disconnected.");
                break; // Exit the main loop
            }
        };

        // Inner loop to process the batch
        loop {
            if !handle_message(
                current_message,
                &app_handle,
                &rt,
                &jellyfin_client,
                &auth_token,
            ) {
                break 'main_loop; // Shutdown received, exit completely
            }

            // Try to get the next message without blocking
            match receiver.try_recv() {
                Ok(next_msg) => {
                    // If we got another message, process it in the next iteration
                    current_message = next_msg;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // The queue is now empty, emit the event and wait for a new message
                    app_handle
                        .emit("download-queue-empty", DownloadQueueEmpty)
                        .unwrap();
                    break; // Exit inner loop, go back to recv()
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break 'main_loop; // Channel disconnected
                }
            }
        }
    }
}
