# Security Model

- HTTP server binds only to `127.0.0.1`.
- `/run` requires `Authorization: Bearer <token>`.
- Prompt length max is 50k characters.
- Tool routing uses strict allowlist (`Tool` enum).
- No arbitrary shell support: runners invoke fixed binaries only.
