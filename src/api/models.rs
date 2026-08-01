use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mihomo version response (/version)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    #[serde(default)]
    pub meta: bool,
    #[serde(default)]
    pub premium: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunConfigInfo {
    #[serde(default)]
    pub enable: bool,
    pub stack: Option<String>,
}

/// Mihomo configuration response (/configs)
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
    pub allow_lan: Option<bool>,
    pub tun: Option<TunConfigInfo>,
}

/// Payload for updating config (PATCH /configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdatePayload {
    pub mode: String,
}

/// Delay history record for a proxy node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayHistory {
    pub time: Option<String>,
    pub delay: u16,
}

/// Proxy node or proxy group info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyItem {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub now: Option<String>,
    pub all: Option<Vec<String>>,
    pub history: Option<Vec<DelayHistory>>,
    pub udp: Option<bool>,
    pub xudp: Option<bool>,
    pub alive: Option<bool>,
}

impl ProxyItem {
    /// Helper to get the latest delay in ms, if available
    pub fn last_delay(&self) -> Option<u16> {
        if let Some(hist) = &self.history {
            if let Some(last) = hist.last() {
                if last.delay > 0 {
                    return Some(last.delay);
                }
            }
        }
        None
    }
}

/// Map of all proxies returned by GET /proxies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxiesResponse {
    pub proxies: HashMap<String, ProxyItem>,
}

/// Delay test response (GET /proxies/{name}/delay)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayResponse {
    pub delay: u16,
}

/// Select node payload (PUT /proxies/{group})
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectProxyPayload {
    pub name: String,
}

/// Connection Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMetadata {
    pub network: Option<String>,
    #[serde(rename = "type")]
    pub conn_type: Option<String>,
    #[serde(rename = "sourceIP")]
    pub source_ip: Option<String>,
    #[serde(rename = "destinationIP")]
    pub destination_ip: Option<String>,
    #[serde(rename = "sourcePort")]
    pub source_port: Option<String>,
    #[serde(rename = "destinationPort")]
    pub destination_port: Option<String>,
    pub host: Option<String>,
    pub process: Option<String>,
    #[serde(rename = "processPath")]
    pub process_path: Option<String>,
    pub rule: Option<String>,
    #[serde(rename = "rulePayload")]
    pub rule_payload: Option<String>,
}

/// Individual Connection info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionItem {
    pub id: String,
    pub metadata: ConnectionMetadata,
    pub upload: u64,
    pub download: u64,
    pub start: Option<String>,
    pub chains: Vec<String>,
    pub rule: Option<String>,
    #[serde(rename = "rulePayload")]
    pub rule_payload: Option<String>,
}

/// Connections list response (GET /connections)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    #[serde(rename = "downloadTotal")]
    pub download_total: u64,
    #[serde(rename = "uploadTotal")]
    pub upload_total: u64,
    pub connections: Vec<ConnectionItem>,
}

/// Real-time traffic WebSocket message (/traffic)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrafficMessage {
    pub up: u64,
    pub down: u64,
}

/// Real-time log WebSocket message (/logs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    #[serde(rename = "type")]
    pub log_type: String,
    pub payload: String,
}

/// Rule item payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleItem {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub payload: String,
    pub proxy: String,
    pub size: Option<usize>,
}

/// Rules response (GET /rules)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesResponse {
    pub rules: Vec<RuleItem>,
}
