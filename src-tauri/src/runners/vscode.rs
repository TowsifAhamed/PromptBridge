use std::process::Command;

pub fn run(prompt: &str) -> Result<(), String> {
    Command::new("code")
        .arg("--new-window")
        .arg(".")
        .spawn()
        .map_err(|e| e.to_string())?;

    println!("Prompt to inject: {prompt}");

    Ok(())
}
