use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const MAX_PROMPT_LEN: usize = 50_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Tool {
    Codex,
    Claude,
    Vscode,
    Continue,
    Clipboard,
    Generic,
}

impl Tool {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tool::Codex => "codex",
            Tool::Claude => "claude",
            Tool::Vscode => "vscode",
            Tool::Continue => "continue",
            Tool::Clipboard => "clipboard",
            Tool::Generic => "generic",
        }
    }
}

impl std::str::FromStr for Tool {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "codex" => Ok(Tool::Codex),
            "claude" => Ok(Tool::Claude),
            "vscode" => Ok(Tool::Vscode),
            "continue" => Ok(Tool::Continue),
            "clipboard" => Ok(Tool::Clipboard),
            "generic" => Ok(Tool::Generic),
            _ => Err(format!("Unsupported tool: {value}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub tool: Tool,
    pub prompt: String,
    pub repo: Option<String>,
    pub cwd: Option<String>,
    pub files: Option<Vec<String>>,
    pub mode: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

impl PromptRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.prompt.trim().is_empty() {
            return Err("prompt is required".into());
        }
        if self.prompt.chars().count() > MAX_PROMPT_LEN {
            return Err(format!("prompt length exceeds {MAX_PROMPT_LEN} characters"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResponse {
    pub ok: bool,
    pub status: String,
    pub message: String,
    pub logs: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiStatus {
    pub last_tool: Option<String>,
    pub last_prompt: Option<String>,
    pub status: String,
    pub last_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{PromptRequest, Tool, MAX_PROMPT_LEN};

    #[test]
    fn rejects_empty_prompt() {
        let req = PromptRequest {
            tool: Tool::Clipboard,
            prompt: "   ".into(),
            repo: None,
            cwd: None,
            files: None,
            mode: None,
            metadata: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn rejects_oversized_prompt() {
        let req = PromptRequest {
            tool: Tool::Clipboard,
            prompt: "x".repeat(MAX_PROMPT_LEN + 1),
            repo: None,
            cwd: None,
            files: None,
            mode: None,
            metadata: None,
        };
        assert!(req.validate().is_err());
    }
}
