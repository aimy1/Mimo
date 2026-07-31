use crate::models::{Proxy, ProxyGroup, Rule};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedProfile {
    pub proxies: Vec<Proxy>,
    pub proxy_groups: Vec<ProxyGroup>,
    pub rules: Vec<Rule>,
    pub has_dns: bool,
    pub has_tun: bool,
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
    pub fn parse_yaml(yaml_content: &str) -> Result<ParsedProfile> {
        let raw: RawClashConfig = serde_yaml::from_str(yaml_content)
            .context("Failed to parse YAML clash config")?;

        let mut proxies = Vec::new();
        if let Some(raw_proxies) = raw.proxies {
            for val in raw_proxies {
                if let Ok(name) = val.get("name").and_then(|v| v.as_str()).context("Missing proxy name") {
                    let ptype = val.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let server = val.get("server").and_then(|v| v.as_str()).unwrap_or("");
                    let port = val.get("port").and_then(|v| v.as_u64()).unwrap_or(0) as u16;
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

        let mut proxy_groups = Vec::new();
        if let Some(raw_groups) = raw.proxy_groups {
            for val in raw_groups {
                if let Ok(name) = val.get("name").and_then(|v| v.as_str()).context("Missing group name") {
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

        let mut rules = Vec::new();
        if let Some(raw_rules) = raw.rules {
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

        Ok(ParsedProfile {
            proxies,
            proxy_groups,
            rules,
            has_dns: raw.dns.is_some(),
            has_tun: raw.tun.is_some(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample_yaml() {
        let sample = r#"
proxies:
  - name: "HK Node 01"
    type: ss
    server: 1.2.3.4
    port: 8388
proxy-groups:
  - name: GLOBAL
    type: select
    proxies:
      - "HK Node 01"
rules:
  - DOMAIN-SUFFIX,google.com,GLOBAL
  - MATCH,DIRECT
"#;
        let parsed = ProfileParser::parse_yaml(sample).unwrap();
        assert_eq!(parsed.proxies.len(), 1);
        assert_eq!(parsed.proxies[0].name, "HK Node 01");
        assert_eq!(parsed.proxy_groups.len(), 1);
        assert_eq!(parsed.rules.len(), 2);
    }
}
