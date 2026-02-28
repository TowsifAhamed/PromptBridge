mod config;
mod http;
mod models;
mod protocol;
mod router;
mod runners;
mod state;
mod utils;

use config::AppConfig;
use models::{PromptRequest, PromptResponse, UiStatus};
use protocol::handler::parse_vibecode_url;
use state::SharedState;

#[tauri::command]
fn handle_protocol(
    url: String,
    state: tauri::State<SharedState>,
) -> Result<PromptResponse, String> {
    let req = parse_vibecode_url(&url)?;
    state.process_request(req)
}

#[tauri::command]
fn submit_request(
    req: PromptRequest,
    state: tauri::State<SharedState>,
) -> Result<PromptResponse, String> {
    state.process_request(req)
}

#[tauri::command]
fn get_ui_status(state: tauri::State<SharedState>) -> Result<UiStatus, String> {
    state
        .ui
        .lock()
        .map_err(|_| "state lock poisoned".to_string())
        .map(|s| s.clone())
}

#[tauri::command]
fn get_settings(state: tauri::State<SharedState>) -> Result<AppConfig, String> {
    state
        .config
        .lock()
        .map_err(|_| "state lock poisoned".to_string())
        .map(|s| s.clone())
}

#[tauri::command]
fn rotate_token(state: tauri::State<SharedState>) -> Result<String, String> {
    let mut cfg = state
        .config
        .lock()
        .map_err(|_| "state lock poisoned".to_string())?;
    let token = cfg.rotate_token();
    cfg.save()?;
    Ok(token)
}

fn main() {
    utils::logger::init();
    let config = AppConfig::load_or_create().expect("failed loading app config");
    let state = SharedState::new(config.clone());

    if config.enable_http_server {
        http::server::spawn_http_server(
            state.clone(),
            config.http_bind.clone(),
            config.http_token.clone(),
            config.cors_allowlist.clone(),
        );
    }

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            handle_protocol,
            submit_request,
            get_ui_status,
            get_settings,
            rotate_token
        ])
        .setup(|app| {
            for arg in std::env::args() {
                if arg.starts_with("vibecode://") {
                    if let Some(state) = app.try_state::<SharedState>() {
                        if let Ok(req) = parse_vibecode_url(&arg) {
                            let _ = state.process_request(req);
                        }
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
