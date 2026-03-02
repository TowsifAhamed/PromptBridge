# PromptBridge v0.1

PromptBridge routes prompts from web links (`vibecode://`) or localhost HTTP fallback to local coding tools.

## Features
- Deep link parsing + startup arg handling for `vibecode://run?...`
- HTTP fallback server: `POST http://127.0.0.1:17777/run` with bearer token
- Tool runners: `codex`, `claude`, `vscode`, `continue`, `clipboard/generic`
- Prompt validation (max 50k chars), strict tool allowlist
- Companion VS Code extension (`/vscode-extension`) for `.vibecode/last_prompt.json`

## Development
```bash
npm install
npm run tauri dev
```

## Test
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

## Protocol quick test
```bash
vibecode://run?tool=clipboard&prompt=hello
```

## HTTP quick test
```bash
curl -X POST http://127.0.0.1:17777/run \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"tool":"clipboard","prompt":"hello"}'
```
