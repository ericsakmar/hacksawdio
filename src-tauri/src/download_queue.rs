use crate::jellyfin::client::JellyfinClient;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
struct DownloadQueueEmpty;

#[derive(Clone, serde::Serialize)]
struct DownloadQueueNotEmpty;

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

// The processor function, to be run in a thread
pub fn process_downloads(
    app_handle: AppHandle,
    receiver: mpsc::Receiver<DownloadQueueMessage>,
    jellyfin_client: Arc<JellyfinClient>,
    auth_token: Arc<Mutex<Option<String>>>,
) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut is_processing = false;

    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(message) => {
                is_processing = true;
                match message {
                    DownloadQueueMessage::NewTrack(track) => {
                        let token_guard = auth_token.lock().unwrap();
                        if let Some(token) = token_guard.as_ref() {
                            let token_for_async = token.clone();
                            rt.block_on(async {
                                if let Err(e) = jellyfin_client
                                    .download_track(
                                        &track.track_id,
                                        &track.track_path,
                                        &token_for_async,
                                    )
                                    .await
                                {
                                    eprintln!("Error downloading track: {}", e);
                                }
                            });
                        } else {
                            eprintln!("Download failed: No auth token available.");
                        }
                    }
                    DownloadQueueMessage::Shutdown => {
                        println!("Download queue shutting down.");
                        break;
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if is_processing {
                    app_handle
                        .emit("download-queue-empty", DownloadQueueEmpty)
                        .unwrap();
                    is_processing = false;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("Download queue channel disconnected.");
                break;
            }
        }
    }
}
