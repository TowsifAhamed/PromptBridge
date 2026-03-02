use crate::models::{PromptRequest, PromptResponse};
use crate::utils::cmd::{command_exists, normalize_cwd};
use std::process::{Command, Stdio};

pub fn run(req: &PromptRequest) -> Result<PromptResponse, String> {
    if !command_exists("codex") {
        return Err("codex CLI not found in PATH".into());
    }

    let mut cmd = Command::new("codex");
    cmd.arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(cwd) = normalize_cwd(req.cwd.as_deref(), req.repo.as_deref()) {
        cmd.current_dir(cwd);
    }

    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(req.prompt.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    Ok(PromptResponse {
        ok: output.status.success(),
        status: if output.status.success() {
            "ok"
        } else {
            "error"
        }
        .into(),
        message: "Codex execution completed".into(),
        logs: vec![
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ],
        next_actions: vec![],
    })
}
