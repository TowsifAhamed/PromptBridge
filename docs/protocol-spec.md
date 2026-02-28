# vibecode:// Protocol Spec

Base URL: `vibecode://run`

Query fields:
- `tool`: `codex|claude|vscode|continue|clipboard|generic`
- `prompt`: prompt string
- `repo`, `cwd`: optional paths
- `files`: comma-separated file list
- `mode`: optional mode string
- `payload`: URL-safe base64 JSON payload (same shape as HTTP body)

Examples:
- `vibecode://run?tool=clipboard&prompt=hello`
- `vibecode://run?payload=eyJ0b29sIjoiY29kZXgiLCJwcm9tcHQiOiJGaXggdGVzdHMifQ`
