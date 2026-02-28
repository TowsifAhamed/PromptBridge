# PromptBridge Architecture

```text
Website SDK -> vibecode://run or POST /run -> Tauri core -> router -> runners
```

Components:
- `protocol::handler`: parses vibecode URLs (query and payload modes).
- `http::server`: localhost-only fallback on `127.0.0.1` with bearer token auth.
- `router::tool_router`: allowlisted tool dispatch.
- `runners/*`: controlled runner implementations (codex/claude/vscode/continue/clipboard).
- `config.rs`: persisted app settings/token.
- `vscode-extension`: companion extension that consumes `.vibecode/last_prompt.json`.
