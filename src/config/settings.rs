use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub language: String,
    pub api_url: String,
    pub secret: Option<String>,
    pub refresh_interval_ms: u64,
    pub http_port: u16,
    pub socks_port: u16,
    pub test_url: String,
    #[serde(default = "default_tun_stack")]
    pub tun_stack: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_true")]
    pub allow_lan: bool,
    #[serde(default = "default_true")]
    pub ipv6: bool,
    #[serde(default = "default_dns_mode")]
    pub dns_mode: String,
    #[serde(default = "default_false")]
    pub auto_sysproxy: bool,
    #[serde(default = "default_sub_update_hours")]
    pub sub_update_hours: u32,
    #[serde(default = "default_ui_theme")]
    pub ui_theme: String,
}

fn default_tun_stack() -> String { "system".to_string() }
fn default_log_level() -> String { "info".to_string() }
fn default_dns_mode() -> String { "fake-ip".to_string() }
fn default_ui_theme() -> String { "Catppuccin".to_string() }
fn default_sub_update_hours() -> u32 { 12 }
fn default_true() -> bool { true }
fn default_false() -> bool { false }

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "zh".to_string(),
            api_url: "http://127.0.0.1:9090".to_string(),
            secret: None,
            refresh_interval_ms: 1000,
            http_port: 7890,
            socks_port: 7891,
            test_url: "http://www.gstatic.com/generate_204".to_string(),
            tun_stack: "system".to_string(),
            log_level: "info".to_string(),
            allow_lan: true,
            ipv6: true,
            dns_mode: "fake-ip".to_string(),
            auto_sysproxy: false,
            sub_update_hours: 12,
            ui_theme: "Catppuccin".to_string(),
        }
    }
}

impl Settings {
    pub fn config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not locate home directory")?;
        let dir = home.join(".config").join("mimo");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }

    pub fn config_file_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_file_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let cfg: Settings = toml::from_str(&content)?;
            Ok(cfg)
        } else {
            let cfg = Self::default();
            cfg.save()?;
            Ok(cfg)
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_file_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
