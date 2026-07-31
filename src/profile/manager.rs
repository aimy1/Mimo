use crate::models::Subscription;
use crate::profile::parser::ProfileParser;
use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileIndex {
    pub active_profile: Option<String>,
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone)]
pub struct ProfileItem {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub file_path: PathBuf,
    pub is_active: bool,
    pub updated_at: Option<u64>,
    pub node_count: usize,
}

pub struct ProfileManager;

impl ProfileManager {
    pub fn config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not locate home directory")?;
        let dir = home.join(".config").join("mimo");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }

    pub fn profiles_dir() -> Result<PathBuf> {
        let dir = Self::config_dir()?.join("profiles");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }

    pub fn index_file_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("index.json"))
    }

    pub fn load_index() -> Result<ProfileIndex> {
        let path = Self::index_file_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let idx: ProfileIndex = serde_json::from_str(&content).unwrap_or_default();
            Ok(idx)
        } else {
            Ok(ProfileIndex::default())
        }
    }

    pub fn save_index(index: &ProfileIndex) -> Result<()> {
        let path = Self::index_file_path()?;
        let content = serde_json::to_string_pretty(index)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub async fn download_profile(name: &str, url: &str) -> Result<PathBuf> {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent("ClashVerge/v1.7.7 Mihomo/v1.18.0 Mimo/0.1.0")
            .build()?;

        let resp = client
            .get(url)
            .send()
            .await
            .context("Network connection error during subscription download")?;

        if !resp.status().is_success() {
            bail!("Subscription server HTTP error status: {}", resp.status());
        }

        let body = resp
            .text()
            .await
            .context("Failed to read subscription response body")?;

        let parsed = ProfileParser::parse_yaml(&body)
            .context("Failed to parse YAML content from downloaded subscription")?;

        let profiles_dir = Self::profiles_dir()?;
        let safe_name = name.replace(['/', '\\', ' ', ':'], "_");
        let file_path = profiles_dir.join(format!("{}.yaml", safe_name));
        fs::write(&file_path, &body)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut index = Self::load_index()?;
        if let Some(sub) = index.subscriptions.iter_mut().find(|s| s.name == name) {
            sub.url = url.to_string();
            sub.updated_time = now;
            sub.file_path = file_path.to_string_lossy().to_string();
            sub.node_count = parsed.proxies.len();
        } else {
            index.subscriptions.push(Subscription {
                id: safe_name.clone(),
                name: name.to_string(),
                url: url.to_string(),
                file_path: file_path.to_string_lossy().to_string(),
                created_time: now,
                updated_time: now,
                node_count: parsed.proxies.len(),
            });
        }

        if index.active_profile.is_none() {
            index.active_profile = Some(name.to_string());
        }

        Self::save_index(&index)?;
        Ok(file_path)
    }

    pub fn list_profiles() -> Result<Vec<ProfileItem>> {
        let index = Self::load_index()?;
        let profiles_dir = Self::profiles_dir()?;
        let mut result = Vec::new();

        if profiles_dir.exists() {
            for entry in fs::read_dir(&profiles_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("default")
                        .to_string();

                    let sub_info = index.subscriptions.iter().find(|s| s.name == stem || s.id == stem);
                    let is_active = index
                        .active_profile
                        .as_ref()
                        .map(|a| a == &stem)
                        .unwrap_or(false);

                    let node_count = sub_info.map(|s| s.node_count).unwrap_or_else(|| {
                        fs::read_to_string(&path)
                            .ok()
                            .and_then(|c| ProfileParser::parse_yaml(&c).ok())
                            .map(|p| p.proxies.len())
                            .unwrap_or(0)
                    });

                    result.push(ProfileItem {
                        id: stem.clone(),
                        name: stem,
                        url: sub_info.map(|s| s.url.clone()),
                        file_path: path,
                        is_active,
                        updated_at: sub_info.map(|s| s.updated_time),
                        node_count,
                    });
                }
            }
        }

        result.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(result)
    }

    pub fn set_active_profile(name: &str) -> Result<()> {
        let mut index = Self::load_index()?;
        index.active_profile = Some(name.to_string());
        Self::save_index(&index)?;
        Ok(())
    }

    pub fn delete_profile(name: &str) -> Result<()> {
        let profiles_dir = Self::profiles_dir()?;
        let safe_name = name.replace(['/', '\\', ' ', ':'], "_");
        let file_path = profiles_dir.join(format!("{}.yaml", safe_name));

        if file_path.exists() {
            fs::remove_file(file_path)?;
        }

        let mut index = Self::load_index()?;
        index.subscriptions.retain(|s| s.name != name && s.id != safe_name);
        if index.active_profile.as_deref() == Some(name) {
            index.active_profile = index.subscriptions.first().map(|s| s.name.clone());
        }

        Self::save_index(&index)?;
        Ok(())
    }
}
