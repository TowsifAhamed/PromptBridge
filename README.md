# VibeCode Router (Starter)

VibeCode Router is a cross-platform desktop router for sending `vibecode://` prompts to local coding tools.

## Architecture

```text
Website / SDK
   ↓  vibecode://run?tool=codex&prompt=...
OS Protocol Handler
   ↓
Tauri App (Rust Core)
   ↓
Tool Router
   ├─ Codex CLI
   ├─ Claude CLI
   ├─ VS Code
   ├─ Continue.dev
   └─ Future agents
```

## Quick start

```bash
npm install
cargo install tauri-cli
cargo tauri dev
```

## Build installers

```bash
cargo tauri build
```

## Protocol example

```text
vibecode://run?tool=codex&prompt=Fix+failing+pytest+cases
```
