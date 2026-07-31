use crate::models::{Proxy, ProxyGroup, Rule};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedProfile {
    pub proxies: Vec<Proxy>,
    pub proxy_groups: Vec<ProxyGroup>,
    pub rules: Vec<Rule>,
    pub has_dns: bool,
    pub has_tun: bool,
    pub raw_yaml: String,
}

#[derive(Debug, Deserialize)]
struct RawClashConfig {
    pub proxies: Option<Vec<serde_yaml::Value>>,
    #[serde(rename = "proxy-groups")]
    pub proxy_groups: Option<Vec<serde_yaml::Value>>,
    pub rules: Option<Vec<String>>,
    pub dns: Option<serde_yaml::Value>,
    pub tun: Option<serde_yaml::Value>,
}

pub struct ProfileParser;

impl ProfileParser {
    pub fn parse_yaml(content: &str) -> Result<ParsedProfile> {
        let content_trimmed = content.trim();

        // 1. Check if content is Base64 encoded
        let decoded_text = if !content_trimmed.contains('\n') && !content_trimmed.contains("proxies:") && !content_trimmed.contains("port:") {
            use base64::Engine;
            let engine = base64::engine::general_purpose::STANDARD;
            if let Ok(bytes) = engine.decode(content_trimmed.as_bytes()) {
                String::from_utf8(bytes).ok()
            } else {
                None
            }
        } else {
            None
        };

        let target_content = decoded_text.as_deref().unwrap_or(content_trimmed);

        // 2. Try parsing YAML
        let raw: Option<RawClashConfig> = serde_yaml::from_str(target_content).ok();

        let mut proxies = Vec::new();
        let mut proxy_groups = Vec::new();
        let mut rules = Vec::new();
        let mut has_dns = false;
        let mut has_tun = false;

        if let Some(raw_config) = raw {
            has_dns = raw_config.dns.is_some();
            has_tun = raw_config.tun.is_some();

            if let Some(raw_proxies) = raw_config.proxies {
                for val in raw_proxies {
                    if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                        let ptype = val.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
                        let server = val.get("server").and_then(|v| v.as_str()).unwrap_or("");
                        let port = val
                            .get("port")
                            .and_then(|v| v.as_u64().or_else(|| v.as_i64().map(|i| i as u64)))
                            .unwrap_or(0) as u16;
                        let udp = val.get("udp").and_then(|v| v.as_bool());
                        let tls = val.get("tls").and_then(|v| v.as_bool());

                        proxies.push(Proxy {
                            name: name.to_string(),
                            proxy_type: ptype.to_string(),
                            server: server.to_string(),
                            port,
                            udp,
                            tls,
                        });
                    }
                }
            }

            if let Some(raw_groups) = raw_config.proxy_groups {
                for val in raw_groups {
                    if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                        let gtype = val.get("type").and_then(|v| v.as_str()).unwrap_or("select");
                        let members: Vec<String> = val
                            .get("proxies")
                            .and_then(|v| v.as_sequence())
                            .map(|seq| {
                                seq.iter()
                                    .filter_map(|s| s.as_str().map(|str_val| str_val.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();

                        proxy_groups.push(ProxyGroup {
                            name: name.to_string(),
                            group_type: gtype.to_string(),
                            proxies: members,
                            now: None,
                        });
                    }
                }
            }

            if let Some(raw_rules) = raw_config.rules {
                for line in raw_rules {
                    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                    if parts.len() >= 3 {
                        rules.push(Rule {
                            rule_type: parts[0].to_string(),
                            payload: parts[1].to_string(),
                            proxy: parts[2].to_string(),
                        });
                    } else if parts.len() == 2 && parts[0].eq_ignore_ascii_case("MATCH") {
                        rules.push(Rule {
                            rule_type: "MATCH".to_string(),
                            payload: String::new(),
                            proxy: parts[1].to_string(),
                        });
                    }
                }
            }
        }

        Ok(ParsedProfile {
            proxies,
            proxy_groups,
            rules,
            has_dns,
            has_tun,
            raw_yaml: target_content.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_riolu() {
        let home = dirs::home_dir().unwrap();
        let path = home.join(".config/mimo/profiles/RioLU.yaml");
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap();
            let parsed = ProfileParser::parse_yaml(&content).unwrap();
            println!("RioLU proxies count: {}", parsed.proxies.len());
            println!("RioLU groups count: {}", parsed.proxy_groups.len());
            for p in &parsed.proxies {
                println!("  Node: {}", p.name);
            }
        }
    }
}
