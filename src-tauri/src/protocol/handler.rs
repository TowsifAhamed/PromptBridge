use crate::router::tool_router::route_tool;
use url::Url;

#[tauri::command]
pub fn handle_protocol(url: String) -> Result<String, String> {
    println!("Incoming URL: {url}");

    let parsed = Url::parse(&url).map_err(|e| e.to_string())?;

    let tool = parsed
        .query_pairs()
        .find(|(k, _)| k == "tool")
        .map(|(_, v)| v.to_string())
        .unwrap_or("codex".to_string());

    let prompt = parsed
        .query_pairs()
        .find(|(k, _)| k == "prompt")
        .map(|(_, v)| v.to_string())
        .unwrap_or_default();

    route_tool(&tool, &prompt)?;

    Ok("Executed".into())
}
