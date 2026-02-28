use crate::models::{PromptRequest, PromptResponse, Tool};
use crate::runners::{claude, clipboard, codex, continue_runner, vscode};

pub fn route_tool(req: &PromptRequest) -> Result<PromptResponse, String> {
    req.validate()?;
    match req.tool {
        Tool::Codex => codex::run(req),
        Tool::Claude => claude::run(req),
        Tool::Vscode => vscode::run(req),
        Tool::Continue => continue_runner::run(req),
        Tool::Clipboard | Tool::Generic => clipboard::run(req),
    }
}

#[cfg(test)]
mod tests {
    use super::route_tool;
    use crate::models::{PromptRequest, Tool};

    #[test]
    fn routes_clipboard() {
        let req = PromptRequest {
            tool: Tool::Clipboard,
            prompt: "hello".into(),
            repo: None,
            cwd: None,
            files: None,
            mode: None,
            metadata: None,
        };
        let result = route_tool(&req);
        assert!(result.is_ok());
    }
}
