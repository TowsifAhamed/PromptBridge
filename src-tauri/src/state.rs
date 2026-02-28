use crate::config::AppConfig;
use crate::models::{PromptRequest, PromptResponse, UiStatus};
use crate::router::tool_router::route_tool;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedState {
    pub config: Arc<Mutex<AppConfig>>,
    pub ui: Arc<Mutex<UiStatus>>,
}

impl SharedState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            ui: Arc::new(Mutex::new(UiStatus {
                last_tool: None,
                last_prompt: None,
                status: "idle".into(),
                last_message: None,
            })),
        }
    }

    pub fn process_request(&self, req: PromptRequest) -> Result<PromptResponse, String> {
        let result = route_tool(&req);
        let mut ui = self.ui.lock().map_err(|_| "state lock poisoned")?;
        ui.last_tool = Some(req.tool.as_str().to_string());
        ui.last_prompt = Some(req.prompt.chars().take(500).collect());
        match &result {
            Ok(resp) => {
                ui.status = resp.status.clone();
                ui.last_message = Some(resp.message.clone());
            }
            Err(err) => {
                ui.status = "error".into();
                ui.last_message = Some(err.clone());
            }
        }
        result
    }
}
