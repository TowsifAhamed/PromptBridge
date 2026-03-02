use crate::utils::paths::config_file_path;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub enable_http_server: bool,
    pub http_bind: String,
    pub http_token: String,
    pub default_tool: String,
    pub default_repo_mode: String,
    pub cors_allowlist: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enable_http_server: true,
            http_bind: "127.0.0.1:17777".into(),
            http_token: new_token(),
            default_tool: "clipboard".into(),
            default_repo_mode: "cwd".into(),
            cors_allowlist: vec![
                "http://localhost:5173".into(),
                "http://127.0.0.1:5173".into(),
            ],
        }
    }
}

impl AppConfig {
    pub fn load_or_create() -> Result<Self, String> {
        let path = config_file_path()?;
        if !path.exists() {
            let cfg = Self::default();
            cfg.save()?;
            return Ok(cfg);
        }

        let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_file_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let data = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, data).map_err(|e| e.to_string())
    }

    pub fn rotate_token(&mut self) -> String {
        self.http_token = new_token();
        self.http_token.clone()
    }
}

fn new_token() -> String {
    uuid::Uuid::new_v4().to_string()
}
