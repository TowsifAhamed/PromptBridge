# Adding Tools

1. Add runner in `src-tauri/src/runners/<tool>.rs` returning `PromptResponse`.
2. Wire in `router/tool_router.rs` and `Tool` enum.
3. Add validation/security checks (no unrestricted shell execution).
4. Document CLI requirements and fallback UX.
