use std::path::Path;
use std::process::{Command, Stdio};

pub fn command_exists(name: &str) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("where")
            .arg(name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    } else {
        Command::new("which")
            .arg(name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

pub fn normalize_cwd(cwd: Option<&str>, repo: Option<&str>) -> Option<String> {
    cwd.or(repo).and_then(|value| {
        let path = Path::new(value);
        if path.exists() {
            Some(path.to_string_lossy().to_string())
        } else {
            None
        }
    })
}
