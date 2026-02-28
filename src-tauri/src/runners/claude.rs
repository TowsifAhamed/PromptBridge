use std::process::Command;

pub fn run(prompt: &str) -> Result<(), String> {
    Command::new("claude")
        .arg(prompt)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
