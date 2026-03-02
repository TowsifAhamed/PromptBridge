use crate::models::{PromptRequest, PromptResponse};

pub fn run(req: &PromptRequest) -> Result<PromptResponse, String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard
        .set_text(req.prompt.clone())
        .map_err(|e| e.to_string())?;

    Ok(PromptResponse {
        ok: true,
        status: "ok".into(),
        message: "Prompt copied to clipboard".into(),
        logs: vec!["clipboard write successful".into()],
        next_actions: vec!["Paste into your preferred coding tool".into()],
    })
}
