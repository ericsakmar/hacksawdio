use serde_json::json;
use tauri::Manager;
use tauri::State;
use tauri_plugin_store::StoreExt;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::jellyfin::client::JellyfinClient;
use crate::jellyfin::models::{AuthResponse, SessionResponse};

mod jellyfin;

pub struct AppState {
    jellyfin_client: JellyfinClient,
    auth_token: Mutex<Option<String>>,
    server_id: Mutex<Option<String>>,
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

    match client.authenticate_user_by_name(&username, &password).await {
        Ok(response) => {
            let mut token_guard = state.auth_token.lock().await;
            *token_guard = Some(response.access_token.clone());

            let mut server_id_guard = state.server_id.lock().await;
            *server_id_guard = Some(response.server_id.clone());

            if let Ok(store) = app_handle.store("store.json") {
                store.set("access_token".to_string(), json!(&response.access_token));
            }

            Ok(response)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let device_id = Uuid::new_v4().to_string();

    let initial_client = JellyfinClient::new(
        "http://192.168.1.153:8097".to_string(),
        "Hacksawdio".to_string(),
        "Hacksawdio Desktop Client".to_string(),
        device_id,
        "0.0.1".to_string(),
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("store.json")?;

            let auth_token = store
                .get("access_token")
                .and_then(|v| v.as_str().map(String::from));

            app.manage(AppState {
                jellyfin_client: initial_client,
                auth_token: Mutex::new(auth_token),
                server_id: Mutex::new(None),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            authenticate_user_by_name_cmd,
            get_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
