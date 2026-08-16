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
    pub external_controller: Option<String>,
    pub secret: Option<String>,
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
    #[serde(rename = "external-controller")]
    pub external_controller: Option<String>,
    pub secret: Option<String>,
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

        // 2. Try parsing as standard Clash YAML
        let raw: Option<RawClashConfig> = serde_yaml::from_str(target_content).ok();

        let mut proxies = Vec::new();
        let mut proxy_groups = Vec::new();
        let mut rules = Vec::new();
        let mut has_dns = false;
        let mut has_tun = false;
        let mut external_controller = None;
        let mut secret = None;

        if let Some(raw_config) = raw {
            has_dns = raw_config.dns.is_some();
            has_tun = raw_config.tun.is_some();
            external_controller = raw_config.external_controller;
            secret = raw_config.secret;

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

        // 3. Fallback: Check if content contains raw URI links (ss://, vmess://, vless://, trojan://, hy2://)
        if proxies.is_empty() {
            let mut uri_proxies = Vec::new();
            let mut yaml_proxy_items = Vec::new();

            for line in target_content.lines() {
                let trimmed = line.trim();
                if let Some(proxy) = Self::parse_single_node_uri(trimmed) {
                    let yaml_item = format!(
                        "  - {{ name: \"{}\", type: {}, server: \"{}\", port: {} }}",
                        proxy.name.replace('"', "\\\""),
                        proxy.proxy_type.to_lowercase(),
                        proxy.server,
                        proxy.port
                    );
                    yaml_proxy_items.push(yaml_item);
                    uri_proxies.push(proxy);
                }
            }

            if !uri_proxies.is_empty() {
                let node_names: Vec<String> = uri_proxies.iter().map(|p| p.name.clone()).collect();
                proxies = uri_proxies;

                proxy_groups.push(ProxyGroup {
                    name: "GLOBAL".to_string(),
                    group_type: "select".to_string(),
                    proxies: node_names.clone(),
                    now: None,
                });
                proxy_groups.push(ProxyGroup {
                    name: "PROXY".to_string(),
                    group_type: "select".to_string(),
                    proxies: node_names.clone(),
                    now: None,
                });

                rules.push(Rule {
                    rule_type: "MATCH".to_string(),
                    payload: String::new(),
                    proxy: "GLOBAL".to_string(),
                });

                // Generate valid composite YAML config for core
                let generated_yaml = format!(
                    "port: 7890\nsocks-port: 7891\nmixed-port: 7897\nallow-lan: true\nmode: rule\nlog-level: info\nexternal-controller: 127.0.0.1:9090\nsecret: \"\"\nproxies:\n{}\nproxy-groups:\n  - name: GLOBAL\n    type: select\n    proxies:\n{}\n  - name: PROXY\n    type: select\n    proxies:\n{}\nrules:\n  - MATCH,GLOBAL\n",
                    yaml_proxy_items.join("\n"),
                    node_names.iter().map(|n| format!("      - \"{}\"", n)).collect::<Vec<_>>().join("\n"),
                    node_names.iter().map(|n| format!("      - \"{}\"", n)).collect::<Vec<_>>().join("\n")
                );

                return Ok(ParsedProfile {
                    proxies,
                    proxy_groups,
                    rules,
                    has_dns: false,
                    has_tun: false,
                    external_controller: Some("127.0.0.1:9090".to_string()),
                    secret: None,
                    raw_yaml: generated_yaml,
                });
            }
        }

        Ok(ParsedProfile {
            proxies,
            proxy_groups,
            rules,
            has_dns,
            has_tun,
            external_controller,
            secret,
            raw_yaml: target_content.to_string(),
        })
    }

    fn parse_single_node_uri(uri: &str) -> Option<Proxy> {
        if uri.starts_with("ss://") {
            let without_scheme = &uri[5..];
            let (encoded_or_userpass, name) = if let Some(idx) = without_scheme.find('#') {
                (&without_scheme[..idx], urlencoding::decode(&without_scheme[idx + 1..]).unwrap_or_default().to_string())
            } else {
                (without_scheme, "Shadowsocks".to_string())
            };

            let name = if name.is_empty() { "Shadowsocks".to_string() } else { name };

            if let Some(at_idx) = encoded_or_userpass.find('@') {
                let host_port = &encoded_or_userpass[at_idx + 1..];
                if let Some(colon) = host_port.rfind(':') {
                    let server = &host_port[..colon];
                    let port = host_port[colon + 1..].parse::<u16>().ok()?;
                    return Some(Proxy {
                        name,
                        proxy_type: "ss".to_string(),
                        server: server.to_string(),
                        port,
                        udp: Some(true),
                        tls: Some(false),
                    });
                }
            }
        } else if uri.starts_with("vmess://") {
            let b64 = &uri[8..];
            use base64::Engine;
            let engine = base64::engine::general_purpose::STANDARD;
            if let Ok(bytes) = engine.decode(b64.as_bytes()) {
                if let Ok(json_val) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    let name = json_val["ps"].as_str().unwrap_or("Vmess").to_string();
                    let server = json_val["add"].as_str().unwrap_or("").to_string();
                    let port = json_val["port"].as_u64().or_else(|| json_val["port"].as_str().and_then(|s| s.parse::<u64>().ok())).unwrap_or(443) as u16;
                    if !server.is_empty() {
                        return Some(Proxy {
                            name,
                            proxy_type: "vmess".to_string(),
                            server,
                            port,
                            udp: Some(true),
                            tls: Some(json_val["tls"].as_str() == Some("tls")),
                        });
                    }
                }
            }
        } else if uri.starts_with("vless://") || uri.starts_with("trojan://") || uri.starts_with("hysteria2://") || uri.starts_with("hy2://") {
            let ptype = if uri.starts_with("vless://") { "vless" } else if uri.starts_with("trojan://") { "trojan" } else { "hysteria2" };
            let prefix_len = if uri.starts_with("hy2://") { 6 } else { ptype.len() + 3 };
            let rest = &uri[prefix_len..];
            let (main_part, name) = if let Some(idx) = rest.find('#') {
                (&rest[..idx], urlencoding::decode(&rest[idx + 1..]).unwrap_or_default().to_string())
            } else {
                (rest, ptype.to_uppercase())
            };

            let name = if name.is_empty() { ptype.to_uppercase() } else { name };

            if let Some(at_idx) = main_part.find('@') {
                let host_port_params = &main_part[at_idx + 1..];
                let host_port = host_port_params.split('?').next().unwrap_or(host_port_params);
                if let Some(colon) = host_port.rfind(':') {
                    let server = &host_port[..colon];
                    let port = host_port[colon + 1..].parse::<u16>().ok()?;
                    return Some(Proxy {
                        name,
                        proxy_type: ptype.to_string(),
                        server: server.to_string(),
                        port,
                        udp: Some(true),
                        tls: Some(true),
                    });
                }
            }
        }

        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_standard_yaml() {
        let yaml_str = r#"
port: 7890
socks-port: 7891
external-controller: 127.0.0.1:9090
secret: "my_secret"
proxies:
  - { name: "HK 01", type: ss, server: 1.1.1.1, port: 443 }
  - { name: "US 01", type: vmess, server: 2.2.2.2, port: 8443 }
proxy-groups:
  - { name: GLOBAL, type: select, proxies: ["HK 01", "US 01"] }
rules:
  - DOMAIN-SUFFIX,google.com,GLOBAL
  - MATCH,DIRECT
"#;
        let parsed = ProfileParser::parse_yaml(yaml_str).unwrap();
        assert_eq!(parsed.proxies.len(), 2);
        assert_eq!(parsed.proxy_groups.len(), 1);
        assert_eq!(parsed.rules.len(), 2);
        assert_eq!(parsed.external_controller.as_deref(), Some("127.0.0.1:9090"));
        assert_eq!(parsed.secret.as_deref(), Some("my_secret"));
    }

    #[test]
    fn test_parse_base64_yaml() {
        use base64::Engine;
        let raw_yaml = "proxies:\n  - { name: \"Node1\", type: ss, server: 1.2.3.4, port: 80 }\nrules:\n  - MATCH,DIRECT\n";
        let encoded = base64::engine::general_purpose::STANDARD.encode(raw_yaml);
        let parsed = ProfileParser::parse_yaml(&encoded).unwrap();
        assert_eq!(parsed.proxies.len(), 1);
        assert_eq!(parsed.proxies[0].name, "Node1");
    }

    #[test]
    fn test_parse_raw_uris() {
        let uri_list = "ss://YWVzLTEyOC1nY206cGFzc3dvcmQ@1.2.3.4:8388#MySSNode\ntrojan://my_password@5.6.7.8:443#MyTrojanNode\n";
        let parsed = ProfileParser::parse_yaml(uri_list).unwrap();
        assert_eq!(parsed.proxies.len(), 2);
        assert_eq!(parsed.proxies[0].name, "MySSNode");
        assert_eq!(parsed.proxies[1].name, "MyTrojanNode");
        assert_eq!(parsed.proxy_groups.len(), 2);
    }
}

