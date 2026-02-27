use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::models::Config;

pub fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Could not determine config directory")?;
    Ok(config_dir.join("everhour").join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    let content = std::fs::read_to_string(&path).context(
        "Could not read config file. Run `everhour login` to set up your API key.",
    )?;
    let config: Config =
        toml::from_str(&content).context("Could not parse config file")?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .context("Could not create config directory")?;
    }
    let content = toml::to_string(config).context("Could not serialize config")?;
    std::fs::write(&path, content).context("Could not write config file")?;
    Ok(())
}
