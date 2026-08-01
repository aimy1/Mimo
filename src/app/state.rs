use crate::config::Config;
use crate::models::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard = 0,
    Proxies = 1,
    Profiles = 2,
    Rules = 3,
    Connections = 4,
    Traffic = 5,
    Logs = 6,
    Settings = 7,
    Privileges = 8,
}

impl Tab {
    pub const ALL: [Tab; 9] = [
        Tab::Dashboard,
        Tab::Proxies,
        Tab::Profiles,
        Tab::Rules,
        Tab::Connections,
        Tab::Traffic,
        Tab::Logs,
        Tab::Settings,
        Tab::Privileges,
    ];

    pub fn title(&self, lang: crate::ui::i18n::Language) -> &'static str {
        match self {
            Tab::Dashboard => crate::ui::i18n::t("tab_dashboard", lang),
            Tab::Proxies => crate::ui::i18n::t("tab_proxies", lang),
            Tab::Profiles => crate::ui::i18n::t("tab_profiles", lang),
            Tab::Rules => crate::ui::i18n::t("tab_rules", lang),
            Tab::Connections => crate::ui::i18n::t("tab_connections", lang),
            Tab::Traffic => crate::ui::i18n::t("tab_traffic", lang),
            Tab::Logs => crate::ui::i18n::t("tab_logs", lang),
            Tab::Settings => crate::ui::i18n::t("tab_settings", lang),
            Tab::Privileges => crate::ui::i18n::t("tab_privileges", lang),
        }
    }
}

/// Main UI Focus Zone: Left Sidebar Navigation vs Main Workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusZone {
    Sidebar,
    Workspace,
}

/// Sub-focus inside Proxies View
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxySubFocus {
    Groups,
    Nodes,
}

pub struct AppState {
    pub active_tab: Tab,
    pub focus_zone: FocusZone,
    pub proxy_sub_focus: ProxySubFocus,
    pub show_help: bool,

    // Hardware & System Metrics
    pub cpu_usage: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,

    // Realtime Search State
    pub is_searching: bool,
    pub search_query: String,

    // Rules Data
    pub rules_resp: Option<crate::models::RulesResponse>,
    pub selected_rule_idx: usize,

    // Settings State
    pub settings_lang: String,
    pub settings_api_url: String,
    pub settings_secret: String,
    pub settings_refresh_ms: u64,
    pub settings_http_port: u16,
    pub settings_socks_port: u16,
    pub settings_test_url: String,
    pub settings_tun_stack: String,
    pub settings_log_level: String,
    pub settings_allow_lan: bool,
    pub settings_ipv6: bool,
    pub settings_focus: usize, // 0..=11 index

    // API Data
    pub version: Option<VersionInfo>,
    pub config: Option<MihomoConfig>,
    pub proxies_resp: Option<ProxiesResponse>,
    pub connections_resp: Option<ConnectionsResponse>,

    // System Proxy & TUN status
    pub is_sysproxy_enabled: bool,
    pub is_tun_enabled: bool,
    pub is_tun_privileged: bool,
    pub tun_interface_name: String,
    pub is_tun_interface_up: bool,
    pub show_tun_modal: bool,
    pub is_granting_privilege: bool,
    pub tun_password_input: String,
    pub tun_input_focus: usize,

    // Profiles & Parsed Active Profile Data
    pub profiles: Vec<crate::profile::ProfileItem>,
    pub parsed_active_profile: Option<crate::profile::ParsedProfile>,
    pub selected_profile_idx: usize,
    pub show_profile_input: bool,
    pub profile_name_input: String,
    pub profile_url_input: String,
    pub profile_input_focus: usize, // 0 for name, 1 for url

    // Proxies Selection Indices
    pub proxy_groups: Vec<String>,
    pub selected_group_idx: usize,
    pub selected_node_idx: usize,
    pub latency_map: HashMap<String, Option<u16>>,

    // Connections Selection Index
    pub selected_conn_idx: usize,

    // Status Error Message
    pub status_error: Option<String>,

    // Realtime Traffic Data
    pub current_traffic: TrafficMessage,
    pub up_history: VecDeque<u64>,
    pub down_history: VecDeque<u64>,

    // Logs Data Stream
    pub logs: VecDeque<LogMessage>,
    pub log_scroll: usize,
    pub log_filter: String,

    // Toast Message
    pub toast: Option<(String, std::time::Instant)>,
}

impl Default for AppState {
    fn default() -> Self {
        let config = Config::load().unwrap_or_default();
        Self {
            active_tab: Tab::Dashboard,
            focus_zone: FocusZone::Workspace,
            proxy_sub_focus: ProxySubFocus::Groups,
            show_help: false,
            cpu_usage: 0.0,
            memory_used_bytes: 0,
            memory_total_bytes: 0,
            is_searching: false,
            search_query: String::new(),
            rules_resp: None,
            selected_rule_idx: 0,
            settings_lang: config.language.clone(),
            settings_api_url: config.api_url.clone(),
            settings_secret: config.secret.clone().unwrap_or_default(),
            settings_refresh_ms: config.refresh_interval_ms,
            settings_http_port: config.http_port,
            settings_socks_port: config.socks_port,
            settings_test_url: config.test_url.clone(),
            settings_tun_stack: config.tun_stack.clone(),
            settings_log_level: config.log_level.clone(),
            settings_allow_lan: config.allow_lan,
            settings_ipv6: config.ipv6,
            settings_focus: 0,
            version: None,
            config: None,
            proxies_resp: None,
            status_error: None,
            is_sysproxy_enabled: crate::core::SystemProxy::is_enabled(),
            is_tun_enabled: false,
            is_tun_privileged: crate::core::TunMode::check_privilege(),
            tun_interface_name: crate::core::TunMode::get_interface_info().0,
            is_tun_interface_up: crate::core::TunMode::get_interface_info().1,
            show_tun_modal: false,
            is_granting_privilege: false,
            tun_password_input: String::new(),
            tun_input_focus: 0,
            profiles: Vec::new(),
            parsed_active_profile: None,
            selected_profile_idx: 0,
            show_profile_input: false,
            profile_name_input: String::new(),
            profile_url_input: String::new(),
            profile_input_focus: 0,
            proxy_groups: Vec::new(),
            selected_group_idx: 0,
            selected_node_idx: 0,
            latency_map: HashMap::new(),
            connections_resp: None,
            selected_conn_idx: 0,
            current_traffic: TrafficMessage::default(),
            up_history: VecDeque::from(vec![0; 40]),
            down_history: VecDeque::from(vec![0; 40]),
            logs: VecDeque::with_capacity(500),
            log_scroll: 0,
            log_filter: "all".to_string(),
            toast: None,
        }
    }
}

impl AppState {
    pub fn push_toast(&mut self, msg: String) {
        self.toast = Some((msg, std::time::Instant::now()));
    }

    pub fn check_toast_expiration(&mut self) {
        if let Some((_, created_at)) = &self.toast {
            if created_at.elapsed() > std::time::Duration::from_secs(4) {
                self.toast = None;
            }
        }
    }

    pub fn selected_group_name(&self) -> Option<&str> {
        self.proxy_groups.get(self.selected_group_idx).map(|s| s.as_str())
    }

    pub fn current_group_nodes(&self) -> Vec<String> {
        let group_name = match self.selected_group_name() {
            Some(g) => g,
            None => return Vec::new(),
        };

        if let Some(resp) = &self.proxies_resp {
            if let Some(group_item) = resp.proxies.get(group_name) {
                if let Some(all) = &group_item.all {
                    if !all.is_empty() {
                        return all.clone();
                    }
                }
            }
        }

        // Fallback to parsed active profile YAML
        if let Some(parsed) = &self.parsed_active_profile {
            if group_name.eq_ignore_ascii_case("GLOBAL") {
                return parsed.proxies.iter().map(|p| p.name.clone()).collect();
            }
            if let Some(g) = parsed.proxy_groups.iter().find(|g| g.name == group_name) {
                return g.proxies.clone();
            }
            return parsed.proxies.iter().map(|p| p.name.clone()).collect();
        }

        Vec::new()
    }

    pub fn filtered_group_nodes(&self) -> Vec<String> {
        let nodes = self.current_group_nodes();
        if self.search_query.trim().is_empty() {
            nodes
        } else {
            let q = self.search_query.trim().to_lowercase();
            nodes.into_iter().filter(|n| n.to_lowercase().contains(&q)).collect()
        }
    }
}
