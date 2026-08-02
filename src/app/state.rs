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
    About = 9,
}

impl Tab {
    pub const ALL: [Tab; 10] = [
        Tab::Dashboard,
        Tab::Proxies,
        Tab::Profiles,
        Tab::Rules,
        Tab::Connections,
        Tab::Traffic,
        Tab::Logs,
        Tab::Settings,
        Tab::Privileges,
        Tab::About,
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
            Tab::About => crate::ui::i18n::t("tab_about", lang),
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
    pub settings_dns_mode: String,
    pub settings_auto_sysproxy: bool,
    pub settings_sub_update_hours: u32,
    pub settings_ui_theme: String,
    pub settings_focus: usize, // 0..=15 index

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
    pub sort_nodes_by_latency: bool,

    // Connections Selection Index
    pub selected_conn_idx: usize,
    pub sort_connections_by_traffic: bool,

    // Status Error & Outbound IP
    pub status_error: Option<String>,
    pub outbound_ip: Option<String>,

    // Realtime Traffic Data
    pub current_traffic: TrafficMessage,
    pub up_history: VecDeque<u64>,
    pub down_history: VecDeque<u64>,

    // Logs Data Stream
    pub logs: VecDeque<LogMessage>,
    pub log_scroll: usize,
    pub log_filter: String,

    // System Info Metadata
    pub sys_hostname: String,
    pub sys_kernel: String,
    pub sys_cpu_brand: String,
    pub sys_cpu_cores: usize,

    // Website Connectivity Latencies (Google, GitHub, YouTube, OpenAI, Bilibili, Baidu)
    pub site_latencies: HashMap<String, Option<u16>>,

    // Toast Message
    pub toast: Option<(String, std::time::Instant)>,
}

impl Default for AppState {
    fn default() -> Self {
        let config = Config::load().unwrap_or_default();

        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let sys_hostname = sysinfo::System::host_name().unwrap_or_else(|| "Linux".into());
        let sys_kernel = sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".into());
        let sys_cpu_brand = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_else(|| "x86_64 CPU".into());
        let sys_cpu_cores = sys.cpus().len();

        let mut site_latencies = HashMap::new();
        site_latencies.insert("Google".to_string(), Some(120));
        site_latencies.insert("GitHub".to_string(), Some(95));
        site_latencies.insert("YouTube".to_string(), Some(110));
        site_latencies.insert("OpenAI".to_string(), Some(140));
        site_latencies.insert("Bilibili".to_string(), Some(18));
        site_latencies.insert("Baidu".to_string(), Some(12));

        Self {
            active_tab: Tab::Dashboard,
            focus_zone: FocusZone::Workspace,
            proxy_sub_focus: ProxySubFocus::Groups,
            show_help: false,
            cpu_usage: 0.0,
            memory_used_bytes: 0,
            memory_total_bytes: 0,
            sys_hostname,
            sys_kernel,
            sys_cpu_brand,
            sys_cpu_cores,
            site_latencies,
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
            settings_dns_mode: config.dns_mode.clone(),
            settings_auto_sysproxy: config.auto_sysproxy,
            settings_sub_update_hours: config.sub_update_hours,
            settings_ui_theme: config.ui_theme.clone(),
            settings_focus: 0,
            version: None,
            config: None,
            proxies_resp: None,
            status_error: None,
            outbound_ip: None,
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
            sort_nodes_by_latency: false,
            connections_resp: None,
            selected_conn_idx: 0,
            sort_connections_by_traffic: false,
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
        if let Some((_, created_at)) = &self.toast
            && created_at.elapsed() > std::time::Duration::from_secs(4) {
                self.toast = None;
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

        if let Some(resp) = &self.proxies_resp
            && let Some(group_item) = resp.proxies.get(group_name)
                && let Some(all) = &group_item.all
                    && !all.is_empty() {
                        return all.clone();
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

    pub fn display_group_nodes(&self) -> Vec<String> {
        let mut nodes = self.filtered_group_nodes();
        if self.sort_nodes_by_latency {
            nodes.sort_by_key(|n| {
                
                self
                    .latency_map
                    .get(n)
                    .copied()
                    .flatten()
                    .or_else(|| {
                        self.proxies_resp
                            .as_ref()
                            .and_then(|r| r.proxies.get(n))
                            .and_then(|p| p.history.as_ref())
                            .and_then(|h| h.last())
                            .and_then(|h| if h.delay > 0 { Some(h.delay) } else { None })
                    })
                    .unwrap_or(u16::MAX)
            });
        }
        nodes
    }

    pub fn filtered_sorted_connections(&self) -> Vec<&crate::models::ConnectionItem> {
        let mut list = Vec::new();
        if let Some(resp) = &self.connections_resp {
            let q = self.search_query.to_lowercase();
            for conn in &resp.connections {
                let host = conn
                    .metadata
                    .host
                    .as_deref()
                    .filter(|h| !h.is_empty())
                    .or(conn.metadata.destination_ip.as_deref())
                    .unwrap_or("Unknown");
                let process = conn.metadata.process.as_deref().unwrap_or("-");

                if q.is_empty()
                    || host.to_lowercase().contains(&q)
                    || process.to_lowercase().contains(&q)
                    || conn.metadata.destination_ip.as_deref().unwrap_or("").contains(&q)
                {
                    list.push(conn);
                }
            }
            if self.sort_connections_by_traffic {
                list.sort_by_key(|c| std::cmp::Reverse(c.download + c.upload));
            }
        }
        list
    }
}
