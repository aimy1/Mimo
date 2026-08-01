use crate::api::models::*;
use anyhow::{bail, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct MihomoClient {
    base_url: String,
    secret: Option<String>,
    client: Client,
}

impl MihomoClient {
    pub fn new(base_url: &str, secret: Option<String>) -> Result<Self> {
        let mut headers = HeaderMap::new();
        if let Some(ref sec) = secret {
            if !sec.trim().is_empty() {
                let auth_val = format!("Bearer {}", sec.trim());
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&auth_val).context("Invalid secret for header")?,
                );
            }
        }

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(5))
            .build()?;

        let mut url = base_url.to_string();
        if url.ends_with('/') {
            url.pop();
        }

        Ok(Self {
            base_url: url,
            secret,
            client,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn secret(&self) -> Option<&str> {
        self.secret.as_deref()
    }

    /// GET /version
    pub async fn get_version(&self) -> Result<VersionInfo> {
        let res = self
            .client
            .get(format!("{}/version", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<VersionInfo>()
            .await?;
        Ok(res)
    }

    /// GET /configs
    pub async fn get_config(&self) -> Result<MihomoConfig> {
        let res = self
            .client
            .get(format!("{}/configs", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<MihomoConfig>()
            .await?;
        Ok(res)
    }

    /// PUT /configs?force=true payload: {"path": "/path/to/config.yaml"}
    pub async fn reload_config(&self, config_path: &str) -> Result<()> {
        let payload = serde_json::json!({
            "path": config_path
        });
        self.client
            .put(format!("{}/configs?force=true", self.base_url))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// PATCH /configs mode
    pub async fn set_mode(&self, mode: &str) -> Result<()> {
        let payload = ConfigUpdatePayload {
            mode: mode.to_string(),
        };
        self.client
            .patch(format!("{}/configs", self.base_url))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// PATCH /configs tun mode
    pub async fn set_tun_enabled(&self, enable: bool) -> Result<()> {
        self.set_tun_config(enable, "system").await
    }

    /// PATCH /configs tun mode with specific stack (system, gvisor, lwip)
    pub async fn set_tun_config(&self, enable: bool, stack: &str) -> Result<()> {
        let payload = serde_json::json!({
            "tun": {
                "enable": enable,
                "stack": stack,
                "auto-route": true,
                "auto-detect-interface": true
            }
        });
        self.client
            .patch(format!("{}/configs", self.base_url))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// GET /proxies
    pub async fn get_proxies(&self) -> Result<ProxiesResponse> {
        let res = self
            .client
            .get(format!("{}/proxies", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<ProxiesResponse>()
            .await?;
        Ok(res)
    }

    /// PUT /proxies/{group}
    pub async fn select_proxy(&self, group: &str, node: &str) -> Result<()> {
        let payload = SelectProxyPayload {
            name: node.to_string(),
        };
        let encoded_group = urlencoding::encode(group);
        self.client
            .put(format!("{}/proxies/{}", self.base_url, encoded_group))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// GET /proxies/{name}/delay
    pub async fn test_delay(
        &self,
        node: &str,
        test_url: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> Result<u16> {
        let encoded_node = urlencoding::encode(node);
        let url_param = test_url.unwrap_or("http://www.gstatic.com/generate_204");
        let timeout_param = timeout_ms.unwrap_or(2000);

        let res = self
            .client
            .get(format!(
                "{}/proxies/{}/delay?url={}&timeout={}",
                self.base_url,
                encoded_node,
                urlencoding::encode(url_param),
                timeout_param
            ))
            .send()
            .await?;

        if !res.status().is_success() {
            bail!("Delay test failed with status {}", res.status());
        }

        let delay_resp = res.json::<DelayResponse>().await?;
        Ok(delay_resp.delay)
    }

    /// GET /connections
    pub async fn get_connections(&self) -> Result<ConnectionsResponse> {
        let res = self
            .client
            .get(format!("{}/connections", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<ConnectionsResponse>()
            .await?;
        Ok(res)
    }

    /// GET /rules
    pub async fn get_rules(&self) -> Result<RulesResponse> {
        let res = self
            .client
            .get(format!("{}/rules", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<RulesResponse>()
            .await?;
        Ok(res)
    }

    /// DELETE /connections/{id}
    pub async fn close_connection(&self, id: &str) -> Result<()> {
        self.client
            .delete(format!("{}/connections/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// DELETE /connections
    pub async fn close_all_connections(&self) -> Result<()> {
        self.client
            .delete(format!("{}/connections", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
