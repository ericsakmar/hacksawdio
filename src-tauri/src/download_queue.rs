use crate::music_manager::MusicManager;
use std::sync::{mpsc, Arc, Mutex};
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
struct DownloadQueueEmpty;

#[derive(Clone, serde::Serialize)]
struct DownloadQueueNotEmpty;

#[derive(Clone, serde::Serialize)]
struct DownloadFailed {
    track_id: String,
    error: String,
}

// Track details for a download
pub struct Track {
    pub track_id: String,
    pub track_path: String,
}

// Message type for the download queue channel
pub enum DownloadQueueMessage {
    NewTrack(Track),
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

    pub fn add_track(&self, track: Track, app_handle: &AppHandle) {
        app_handle
            .emit("download-queue-not-empty", DownloadQueueNotEmpty)
            .unwrap();
        self.sender
            .send(DownloadQueueMessage::NewTrack(track))
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
    jellyfin_client: &Arc<MusicManager>,
    auth_token: &Arc<Mutex<Option<String>>>,
) -> bool {
    // returns false if shutdown
    match message {
        DownloadQueueMessage::NewTrack(track) => {
            let token = {
                let token_guard = auth_token.lock().unwrap();
                token_guard.clone()
            };

            if let Some(token) = token {
                rt.block_on(async {
                    if let Err(e) = jellyfin_client
                        .download_track(&track.track_id, &track.track_path, &token)
                        .await
                    {
                        let error_message = e.to_string();
                        eprintln!(
                            "Error downloading track {}: {}",
                            &track.track_id, &error_message
                        );
                        app_handle
                            .emit(
                                "download-failed",
                                DownloadFailed {
                                    track_id: track.track_id,
                                    error: error_message,
                                },
                            )
                            .unwrap();
                    }
                });
            } else {
                let error_message = "Download failed: No auth token available.".to_string();
                eprintln!("{}", &error_message);
                app_handle
                    .emit(
                        "download-failed",
                        DownloadFailed {
                            track_id: track.track_id,
                            error: error_message,
                        },
                    )
                    .unwrap();
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

    loop {
        match receiver.recv() {
            Ok(message) => {
                if !handle_message(message, &app_handle, &rt, &jellyfin_client, &auth_token) {
                    break;
                }

                // Process all other pending messages in the queue
                while let Ok(message) = receiver.try_recv() {
                    if !handle_message(message, &app_handle, &rt, &jellyfin_client, &auth_token) {
                        break; // Shutdown message received
                    }
                }

                // After processing all items, emit the empty event
                app_handle
                    .emit("download-queue-empty", DownloadQueueEmpty)
                    .unwrap();
            }
            Err(mpsc::RecvError) => {
                // This error occurs if the sender has been dropped.
                println!("Download queue channel disconnected.");
                break;
            }
        }
    }
}
