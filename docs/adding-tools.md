# Adding New Tools

1. Create a runner in `src-tauri/src/runners`.
2. Implement:

   ```rust
   pub fn run(prompt: &str) -> Result<(), String>
   ```

3. Register the runner in `src-tauri/src/router/tool_router.rs`.
4. Rebuild the app.
