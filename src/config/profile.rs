use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileItem {
    pub name: String,
    pub url: Option<String>,
    pub file_path: PathBuf,
    pub updated_at: Option<u64>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileMeta {
    pub active_profile: Option<String>,
    pub profiles: Vec<ProfileItem>,
}

pub struct ProfileManager;

impl ProfileManager {
    pub fn profiles_dir() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .context("Failed to get config directory")?
            .join("mimo")
            .join("profiles");

        if !dir.exists() {
            fs::create_dir_all(&dir).context("Failed to create profiles directory")?;
        }

        Ok(dir)
    }

    pub fn meta_file_path() -> Result<PathBuf> {
        Ok(Self::profiles_dir()?.join("index.json"))
    }

    pub fn load_meta() -> Result<ProfileMeta> {
        let meta_path = Self::meta_file_path()?;
        if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            let meta: ProfileMeta = serde_json::from_str(&content)?;
            Ok(meta)
        } else {
            Ok(ProfileMeta::default())
        }
    }

    pub fn save_meta(meta: &ProfileMeta) -> Result<()> {
        let meta_path = Self::meta_file_path()?;
        let content = serde_json::to_string_pretty(meta)?;
        fs::write(meta_path, content)?;
        Ok(())
    }

    pub fn list_profiles() -> Result<Vec<ProfileItem>> {
        let mut meta = Self::load_meta()?;
        let profiles_dir = Self::profiles_dir()?;

        // Scan profiles directory for YAML files
        let mut found_profiles = Vec::new();

        if let Ok(entries) = fs::read_dir(&profiles_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "yaml" || ext == "yml" {
                            let file_stem = path
                                .file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();

                            let url = meta
                                .profiles
                                .iter()
                                .find(|p| p.name == file_stem)
                                .and_then(|p| p.url.clone());

                            let updated_at = entry
                                .metadata()
                                .ok()
                                .and_then(|m| m.modified().ok())
                                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs());

                            let is_active = meta
                                .active_profile
                                .as_ref()
                                .map(|a| a == &file_stem)
                                .unwrap_or(false);

                            found_profiles.push(ProfileItem {
                                name: file_stem,
                                url,
                                file_path: path,
                                updated_at,
                                is_active,
                            });
                        }
                    }
                }
            }
        }

        found_profiles.sort_by(|a, b| a.name.cmp(&b.name));

        // Sync with index.json
        meta.profiles = found_profiles.clone();
        let _ = Self::save_meta(&meta);

        Ok(found_profiles)
    }

    pub async fn download_profile(name: &str, url: &str) -> Result<PathBuf> {
        let dir = Self::profiles_dir()?;
        let file_path = dir.join(format!("{}.yaml", name));

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()?;

        let response = client
            .get(url)
            .header("User-Agent", "clash-verge/v1.6.0")
            .send()
            .await?
            .error_for_status()?;

        let content = response.text().await?;

        // Basic YAML validation check
        if !content.contains("proxies") && !content.contains("proxy-groups") && !content.contains("rules") {
            anyhow::bail!("Downloaded content does not appear to be a valid Clash/Mihomo YAML configuration.");
        }

        fs::write(&file_path, content)?;

        let mut meta = Self::load_meta()?;
        if let Some(existing) = meta.profiles.iter_mut().find(|p| p.name == name) {
            existing.url = Some(url.to_string());
            existing.file_path = file_path.clone();
            existing.updated_at = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs());
        } else {
            meta.profiles.push(ProfileItem {
                name: name.to_string(),
                url: Some(url.to_string()),
                file_path: file_path.clone(),
                updated_at: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs()),
                is_active: false,
            });
        }

        Self::save_meta(&meta)?;

        Ok(file_path)
    }

    pub fn set_active_profile(name: &str) -> Result<()> {
        let mut meta = Self::load_meta()?;
        meta.active_profile = Some(name.to_string());
        for p in &mut meta.profiles {
            p.is_active = p.name == name;
        }
        Self::save_meta(&meta)?;
        Ok(())
    }

    pub fn delete_profile(name: &str) -> Result<()> {
        let dir = Self::profiles_dir()?;
        let file_path = dir.join(format!("{}.yaml", name));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        let mut meta = Self::load_meta()?;
        meta.profiles.retain(|p| p.name != name);
        if meta.active_profile.as_deref() == Some(name) {
            meta.active_profile = None;
        }
        Self::save_meta(&meta)?;
        Ok(())
    }
}
