use std::{error::Error, fs, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub collection_name: String,
    pub request_timeout: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            collection_name: "default".to_string(),
            request_timeout: 5,
        }
    }
}

pub fn get_config_dir() -> Option<PathBuf> {
    let proj = ProjectDirs::from("com", "you", "reqtui")?;
    Some(proj.config_dir().to_path_buf())
}

pub fn get_collections_dir() -> Option<PathBuf> {
    Some(get_config_dir()?.join("collections"))
}

pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let Some(config_dir) = get_config_dir() else {
        return Err("Could not find config directory".into());
    };

    fs::create_dir_all(&config_dir)?;

    let config_path = config_dir.join("config.json");

    if !config_path.exists() {
        let default_config = AppConfig::default();
        let json_config = serde_json::to_string_pretty(&default_config)?;
        fs::write(&config_path, json_config)?;
        return Ok(default_config);
    }

    let contents = fs::read_to_string(&config_path)?;
    let config: AppConfig = serde_json::from_str(&contents)?;
    Ok(config)
}
