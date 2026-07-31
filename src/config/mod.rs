pub mod profile;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub secret: Option<String>,
    pub refresh_interval_ms: u64,
    pub language: String,
    pub http_port: u16,
    pub socks_port: u16,
    pub test_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "http://127.0.0.1:9090".to_string(),
            secret: None,
            refresh_interval_ms: 1000,
            language: "zh".to_string(),
            http_port: 7890,
            socks_port: 7891,
            test_url: "http://www.gstatic.com/generate_204".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Self::default();

        // 1. Try reading ~/.config/mimo/config.toml
        if let Some(config_path) = Self::config_file_path() {
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(parsed) = toml::from_str::<Config>(&content) {
                        config = parsed;
                    }
                }
            }
        }

        // 2. Override with Environment Variables if set
        if let Ok(env_url) = std::env::var("MIHOMO_API") {
            if !env_url.trim().is_empty() {
                config.api_url = env_url;
            }
        }
        if let Ok(env_secret) = std::env::var("MIHOMO_SECRET") {
            if !env_secret.trim().is_empty() {
                config.secret = Some(env_secret);
            }
        }

        // Ensure api_url doesn't have trailing slash
        if config.api_url.ends_with('/') {
            config.api_url.pop();
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(config_path) = Self::config_file_path() {
            if let Some(parent) = config_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let content = toml::to_string_pretty(self)?;
            fs::write(config_path, content)?;
        }
        Ok(())
    }

    pub fn config_file_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("mimo").join("config.toml"))
    }
}
