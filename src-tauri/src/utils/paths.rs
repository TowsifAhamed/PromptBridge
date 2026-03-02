use std::path::PathBuf;

pub fn app_dir() -> Result<PathBuf, String> {
    let base = dirs::config_dir().ok_or("unable to locate config directory")?;
    Ok(base.join("promptbridge"))
}

pub fn config_file_path() -> Result<PathBuf, String> {
    Ok(app_dir()?.join("config.json"))
}
