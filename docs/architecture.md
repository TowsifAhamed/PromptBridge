# Architecture

VibeCode Router resolves `vibecode://` URLs and dispatches prompts to local tool runners.

1. Website/SDK generates protocol URL.
2. OS launches VibeCode Router.
3. Rust protocol parser extracts route params.
4. Tool router dispatches to matching runner.
5. Runner launches tool CLI/editor.
