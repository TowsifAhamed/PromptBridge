use std::process::Command;

pub fn run(prompt: &str) -> Result<(), String> {
    println!("Running Codex with prompt: {prompt}");

    Command::new("codex")
        .arg("run")
        .arg(prompt)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
