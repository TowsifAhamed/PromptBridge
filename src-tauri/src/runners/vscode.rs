use crate::models::{PromptRequest, PromptResponse};
use crate::utils::cmd::{command_exists, normalize_cwd};
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn run(req: &PromptRequest) -> Result<PromptResponse, String> {
    if !command_exists("code") {
        return Err("VS Code CLI `code` not found in PATH".into());
    }

    let cwd = normalize_cwd(req.cwd.as_deref(), req.repo.as_deref()).unwrap_or_else(|| ".".into());
    let repo_path = PathBuf::from(cwd);
    let vibecode_dir = repo_path.join(".vibecode");
    fs::create_dir_all(&vibecode_dir).map_err(|e| e.to_string())?;
    let payload = json!({
      "tool": req.tool.as_str(),
      "prompt": req.prompt,
      "files": req.files,
      "metadata": req.metadata,
    });
    fs::write(
        vibecode_dir.join("last_prompt.json"),
        serde_json::to_vec_pretty(&payload).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;

    Command::new("code")
        .arg("--new-window")
        .arg(&repo_path)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(PromptResponse {
        ok: true,
        status: "ok".into(),
        message: "Opened VS Code and dropped .vibecode/last_prompt.json".into(),
        logs: vec![format!("repo: {}", repo_path.display())],
        next_actions: vec![
            "Install PromptBridge VS Code extension and run: VibeCode: Consume Last Prompt".into(),
        ],
    })
}
