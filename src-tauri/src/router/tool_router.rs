use crate::runners::{claude, codex, continue_runner, vscode};

pub fn route_tool(tool: &str, prompt: &str) -> Result<(), String> {
    match tool {
        "codex" => codex::run(prompt),
        "claude" => claude::run(prompt),
        "vscode" => vscode::run(prompt),
        "continue" => continue_runner::run(prompt),
        _ => Err(format!("Unknown tool: {tool}")),
    }
}
