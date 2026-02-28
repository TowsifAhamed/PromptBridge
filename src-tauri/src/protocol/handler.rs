use crate::models::{PromptRequest, Tool};
use base64::Engine;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
struct PayloadRequest {
    tool: Option<String>,
    prompt: Option<String>,
    repo: Option<String>,
    cwd: Option<String>,
    files: Option<Vec<String>>,
    mode: Option<String>,
    metadata: Option<std::collections::HashMap<String, String>>,
}

pub fn parse_vibecode_url(url: &str) -> Result<PromptRequest, String> {
    let parsed = Url::parse(url).map_err(|e| e.to_string())?;
    if parsed.scheme() != "vibecode" {
        return Err("invalid scheme, expected vibecode://".into());
    }

    let mut tool = None;
    let mut prompt = None;
    let mut repo = None;
    let mut cwd = None;
    let mut files: Option<Vec<String>> = None;
    let mut mode = None;
    let mut metadata = None;

    for (k, v) in parsed.query_pairs() {
        match k.as_ref() {
            "tool" => tool = Some(v.to_string()),
            "prompt" => prompt = Some(v.to_string()),
            "repo" => repo = Some(v.to_string()),
            "cwd" => cwd = Some(v.to_string()),
            "files" => {
                files = Some(
                    v.split(',')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect(),
                )
            }
            "mode" => mode = Some(v.to_string()),
            "payload" => {
                let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(v.as_bytes())
                    .map_err(|e| e.to_string())?;
                let payload: PayloadRequest =
                    serde_json::from_slice(&decoded).map_err(|e| e.to_string())?;
                tool = payload.tool.or(tool);
                prompt = payload.prompt.or(prompt);
                repo = payload.repo.or(repo);
                cwd = payload.cwd.or(cwd);
                files = payload.files.or(files);
                mode = payload.mode.or(mode);
                metadata = payload.metadata.or(metadata);
            }
            _ => {}
        }
    }

    let tool_value = tool.unwrap_or_else(|| "clipboard".to_string());
    let request = PromptRequest {
        tool: tool_value.parse::<Tool>()?,
        prompt: prompt.unwrap_or_default(),
        repo,
        cwd,
        files,
        mode,
        metadata,
    };
    request.validate()?;
    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::parse_vibecode_url;
    use crate::models::Tool;

    #[test]
    fn parses_query() {
        let req =
            parse_vibecode_url("vibecode://run?tool=clipboard&prompt=hello&files=a,b").unwrap();
        assert_eq!(req.tool, Tool::Clipboard);
        assert_eq!(req.prompt, "hello");
        assert_eq!(req.files.unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn parses_payload() {
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(r#"{"tool":"codex","prompt":"from payload"}"#);
        let req = parse_vibecode_url(&format!("vibecode://run?payload={payload}")).unwrap();
        assert_eq!(req.tool, Tool::Codex);
        assert_eq!(req.prompt, "from payload");
    }
}
