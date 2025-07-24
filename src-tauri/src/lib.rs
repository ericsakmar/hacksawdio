use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::jellyfin::client::JellyfinClient;
use crate::jellyfin::models::AuthResponse;

mod jellyfin;

pub struct AppState {
    jellyfin_client: JellyfinClient,
    auth_token: Mutex<Option<String>>,
    server_id: Mutex<Option<String>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn authenticate_user_by_name_cmd(
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
        "TauriJellyfinApp".to_string(),
        "My Tauri Desktop Client".to_string(),
        device_id,
        "1.0.0".to_string(),
    );

    tauri::Builder::default()
        .manage(AppState {
            jellyfin_client: initial_client,
            auth_token: Mutex::new(None), // No token on startup
            server_id: Mutex::new(None),  // No server ID on startup
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            authenticate_user_by_name_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
