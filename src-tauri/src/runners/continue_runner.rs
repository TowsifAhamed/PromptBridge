use crate::models::{PromptRequest, PromptResponse};
use crate::runners::{clipboard, vscode};

pub fn run(req: &PromptRequest) -> Result<PromptResponse, String> {
    let mut response = vscode::run(req).or_else(|_| clipboard::run(req))?;
    response
        .next_actions
        .push("Inside VS Code, use PromptBridge panel and forward to Continue.".into());
    Ok(response)
}
