use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub token: String,
    pub organization: String,
    pub project: String,
    pub refresh_interval: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            token: String::new(),
            organization: String::new(),
            project: String::new(),
            refresh_interval: 60,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("glm-usage-tray");
    path.push("config.json");
    path
}

pub fn load_config() -> Option<ApiConfig> {
    let path = get_config_path();
    if path.exists() {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
    } else {
        None
    }
}

pub fn save_config(config: &ApiConfig) -> Result<(), String> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn is_config_valid(config: &ApiConfig) -> bool {
    !config.token.is_empty()
        && !config.organization.is_empty()
        && !config.project.is_empty()
}
