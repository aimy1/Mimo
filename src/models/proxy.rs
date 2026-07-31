use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Subscription metadata item stored in ~/.config/mimo/index.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub url: String,
    pub file_path: String,
    pub created_time: u64,
    pub updated_time: u64,
    pub node_count: usize,
}

/// Internal Proxy Node model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub server: String,
    pub port: u16,
    pub udp: Option<bool>,
    pub tls: Option<bool>,
}

/// Internal Proxy Group model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyGroup {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub proxies: Vec<String>,
    pub now: Option<String>,
}

/// Internal Rule model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub payload: String,
    pub proxy: String,
}

/// Mihomo REST API Proxy Item detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiProxyItem {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub udp: Option<bool>,
    pub history: Option<Vec<ApiDelayHistory>>,
    pub all: Option<Vec<String>>,
    pub now: Option<String>,
}

impl ApiProxyItem {
    pub fn last_delay(&self) -> Option<u16> {
        self.history.as_ref().and_then(|h| h.last()).and_then(|item| {
            if item.delay > 0 {
                Some(item.delay)
            } else {
                None
            }
        })
    }

    pub fn is_group(&self) -> bool {
        self.proxy_type.eq_ignore_ascii_case("Selector")
            || self.proxy_type.eq_ignore_ascii_case("URLTest")
            || self.proxy_type.eq_ignore_ascii_case("Fallback")
            || self.proxy_type.eq_ignore_ascii_case("LoadBalance")
            || self.all.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDelayHistory {
    pub time: String,
    pub delay: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxiesResponse {
    pub proxies: HashMap<String, ApiProxyItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MihomoConfig {
    pub mode: Option<String>,
    #[serde(rename = "port")]
    pub http_port: Option<u16>,
    #[serde(rename = "socks-port")]
    pub socks_port: Option<u16>,
    #[serde(rename = "mixed-port")]
    pub mixed_port: Option<u16>,
    #[serde(rename = "log-level")]
    pub log_level: Option<String>,
    #[serde(rename = "allow-lan")]
    pub allow_lan: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub meta: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrafficMessage {
    pub up: u64,
    pub down: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    #[serde(rename = "type")]
    pub log_type: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionItem {
    pub id: String,
    pub metadata: ConnectionMetadata,
    pub upload: u64,
    pub download: u64,
    pub start: String,
    pub chains: Vec<String>,
    pub rule: Option<String>,
    #[serde(rename = "rulePayload")]
    pub rule_payload: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMetadata {
    #[serde(rename = "network")]
    pub network_type: String,
    pub host: Option<String>,
    #[serde(rename = "destinationIP")]
    pub destination_ip: Option<String>,
    #[serde(rename = "destinationPort")]
    pub destination_port: Option<String>,
    pub process: Option<String>,
    #[serde(rename = "processPath")]
    pub process_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    pub connections: Vec<ConnectionItem>,
    #[serde(rename = "uploadTotal")]
    pub upload_total: u64,
    #[serde(rename = "downloadTotal")]
    pub download_total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesResponse {
    pub rules: Vec<Rule>,
}
