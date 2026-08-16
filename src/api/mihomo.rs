use crate::models::*;
use anyhow::{Context, Result};
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
        if let Some(ref sec) = secret
            && !sec.trim().is_empty() {
                let auth_val = format!("Bearer {}", sec.trim());
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&auth_val).context("Invalid secret for header")?,
                );
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
        let payload = serde_json::json!({
            "mode": mode
        });
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
        self.set_tun_config(enable, "gvisor").await
    }

    /// PATCH /configs tun mode with specific stack (system, gvisor, lwip)
    pub async fn set_tun_config(&self, enable: bool, stack: &str) -> Result<()> {
        let payload = if enable {
            serde_json::json!({
                "tun": {
                    "enable": true,
                    "stack": stack,
                    "auto-route": true,
                    "auto-detect-interface": true
                }
            })
        } else {
            serde_json::json!({
                "tun": {
                    "enable": false
                }
            })
        };
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
        let payload = serde_json::json!({
            "name": node
        });
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

        let url = format!(
            "{}/proxies/{}/delay?url={}&timeout={}",
            self.base_url,
            encoded_node,
            urlencoding::encode(url_param),
            timeout_param
        );

        #[derive(serde::Deserialize)]
        struct DelayResult {
            delay: u16,
        }

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<DelayResult>()
            .await?;

        Ok(res.delay)
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

    /// GET /group/{name}/delay
    pub async fn test_group_delay(
        &self,
        group: &str,
        test_url: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> Result<std::collections::HashMap<String, u16>> {
        let encoded_group = urlencoding::encode(group);
        let url_param = test_url.unwrap_or("http://www.gstatic.com/generate_204");
        let timeout_param = timeout_ms.unwrap_or(3000);

        let url = format!(
            "{}/group/{}/delay?url={}&timeout={}",
            self.base_url,
            encoded_group,
            urlencoding::encode(url_param),
            timeout_param
        );

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<std::collections::HashMap<String, u16>>()
            .await?;

        Ok(res)
    }

    /// Fetch public outbound IP address with multi-endpoint fallback
    pub async fn get_outbound_ip(proxy_port: Option<u16>) -> Result<String> {
        let endpoints = [
            "https://api.ipify.org?format=json",
            "https://api64.ipify.org?format=json",
            "https://ip.sb",
            "https://ifconfig.me/ip",
            "https://icanhazip.com",
        ];

        let mut builder = Client::builder().timeout(Duration::from_secs(3));
        if let Some(port) = proxy_port {
            if let Ok(proxy) = reqwest::Proxy::all(format!("http://127.0.0.1:{}", port)) {
                builder = builder.proxy(proxy);
            }
        }
        let client = builder.build().unwrap_or_default();

        for url in endpoints {
            if let Ok(resp) = client.get(url).send().await {
                if resp.status().is_success() {
                    if let Ok(text) = resp.text().await {
                        let trimmed = text.trim();
                        if trimmed.starts_with('{') {
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                                if let Some(ip) = val["ip"].as_str() {
                                    return Ok(ip.trim().to_string());
                                }
                            }
                        } else if !trimmed.is_empty() && trimmed.len() <= 45 && (trimmed.contains('.') || trimmed.contains(':')) {
                            return Ok(trimmed.to_string());
                        }
                    }
                }
            }
        }

        anyhow::bail!("All IP diagnosis endpoints failed")
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

