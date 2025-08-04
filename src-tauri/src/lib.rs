use serde_json::json;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::Manager;
use tauri::State;
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

use crate::download_queue::{process_downloads, DownloadQueue};
use crate::jellyfin::client::JellyfinClient;
use crate::jellyfin::models::{AlbumSearchResponse, AuthResponse, SessionResponse};

use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

mod db;

mod download_queue;
mod jellyfin;
mod models;
mod schema;

pub struct AppState {
    jellyfin_client: Arc<JellyfinClient>,
    auth_token: Arc<Mutex<Option<String>>>,
    download_queue: DownloadQueue,
}

async fn set_access_token(
    app_handle: &tauri::AppHandle,
    state: &State<'_, AppState>,
    access_token: &str,
) {
    let mut token_guard = state.auth_token.lock().unwrap();
    *token_guard = Some(access_token.to_string());

    if let Ok(store) = app_handle.store("store.json") {
        store.set("access_token", json!(access_token));
    }
}

async fn get_access_token(state: &State<'_, AppState>) -> Result<String, String> {
    let token_guard = state.auth_token.lock().unwrap();

    token_guard
        .clone()
        .ok_or_else(|| "Unauthorized".to_string())
}

#[tauri::command]
fn get_session(app_handle: tauri::AppHandle) -> Result<SessionResponse, String> {
    let store = app_handle
        .store("store.json")
        .map_err(|e| format!("Failed to access store: {}", e))?;

    let authenticated = store
        .get("access_token")
        .and_then(|v| v.as_str().map(String::from))
        .map_or(false, |token| !token.is_empty());

    Ok(SessionResponse { authenticated })
}

#[tauri::command]
async fn authenticate_user_by_name_cmd(
    app_handle: tauri::AppHandle,
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<AuthResponse, String> {
    let client = &state.jellyfin_client;

    let response = client
        .authenticate_user_by_name(&username, &password)
        .await
        .map_err(|e| e.to_string())?;

    set_access_token(&app_handle, &state, &response.access_token).await;

    Ok(response)
}

#[tauri::command]
async fn search_albums(
    search: String,
    limit: Option<u32>,
    offset: Option<u32>,
    state: State<'_, AppState>,
) -> Result<AlbumSearchResponse, String> {
    let client = &state.jellyfin_client;

    let access_token = get_access_token(&state).await?;

    client
        .search_albums(&search, &access_token, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn download_album(
    app_handle: tauri::AppHandle,
    album_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let client = &state.jellyfin_client;

    let access_token = get_access_token(&state).await?;

    client
        .download_album(&app_handle, &album_id, &access_token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_album(album_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let client = &state.jellyfin_client;

    client
        .delete_album(&album_id)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let device_id = Uuid::new_v4().to_string();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("store.json")?;

            let auth_token = Arc::new(Mutex::new(
                store
                    .get("access_token")
                    .and_then(|v| v.as_str().map(String::from)),
            ));

            let db_pool = db::establish_connection();

            let (download_queue, download_receiver) = DownloadQueue::new();

            let jellyfin_client = Arc::new(JellyfinClient::new(
                "http://192.168.1.153:8097".to_string(),
                "Hacksawdio".to_string(),
                "Hacksawdio Desktop Client".to_string(),
                device_id,
                "0.0.1".to_string(),
                db_pool,
                download_queue.clone(),
            ));

            let app_handle = app.handle().clone();
            let jellyfin_client_clone = jellyfin_client.clone();
            let auth_token_clone = auth_token.clone();

            thread::spawn(move || {
                process_downloads(
                    app_handle,
                    download_receiver,
                    jellyfin_client_clone,
                    auth_token_clone,
                );
            });

            app.manage(AppState {
                jellyfin_client,
                auth_token,
                download_queue,
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Destroyed => {
                let state: tauri::State<AppState> = window.state();
                state.download_queue.shutdown();
            }
            _ => {}
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            authenticate_user_by_name_cmd,
            get_session,
            search_albums,
            download_album,
            delete_album
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
