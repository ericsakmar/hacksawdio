use crate::jellyfin::client::JellyfinClient;
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread,
};
use tauri::Emitter;

use tauri::AppHandle;

#[derive(Clone, serde::Serialize)]
struct DownloadQueueEmpty;

#[derive(Clone, serde::Serialize)]
struct DownloadQueueNotEmpty;

pub struct Track {
    pub track_id: String,
    pub track_path: String,
}

pub struct DownloadQueue {
    queue: Arc<(Mutex<VecDeque<Track>>, Condvar)>,
}

impl DownloadQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    pub fn add_track(&self, track: Track, app_handle: &AppHandle) {
        let (lock, cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        let is_empty = queue.is_empty();
        if is_empty {
            app_handle
                .emit("download-queue-not-empty", DownloadQueueNotEmpty)
                .unwrap();
        }
        queue.push_back(track);
        cvar.notify_one();
    }

    // more like start_queue
    pub fn process_queue(
        &self,
        app_handle: AppHandle,
        jellyfin_client: JellyfinClient,
        auth_token: Option<String>,
    ) {
        let queue_clone = self.queue.clone();
        thread::spawn(move || loop {
            let (lock, cvar) = &*queue_clone;
            let mut queue = lock.lock().unwrap();
            while let Some(track) = queue.pop_front() {
                let token_guard = auth_token.lock().unwrap();
                if let Some(token) = token_guard.as_ref() {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        if let Err(e) = jellyfin_client
                            .download_track(&track.track_id, &track.track_path, &token)
                            .await
                        {
                            eprintln!("Error downloading track: {}", e);
                        }
                    });
                }
            }

            app_handle
                .emit("download-queue-empty", DownloadQueueEmpty)
                .unwrap();

            queue = cvar.wait(queue).unwrap();
        });
    }
}

