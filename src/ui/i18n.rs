use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
#[derive(Default)]
pub enum Language {
    #[default]
    Zh,
    En,
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "en" | "english" => Language::En,
            _ => Language::Zh,
        }
    }
}

pub fn t(key: &str, lang: Language) -> &str {
    match lang {
        Language::Zh => match key {
            "tab_dashboard" => " 仪表盘 ",
            "tab_proxies" => " 节点选择 ",
            "tab_profiles" => " 订阅配置 ",
            "tab_rules" => " 规则路由 ",
            "tab_connections" => " 连接管理 ",
            "tab_traffic" => " 流量监控 ",
            "tab_logs" => " 运行日志 ",
            "tab_settings" => " 系统设置 ",
            "tab_privileges" => " 权限管理 ",
            "tab_about" => " 关于 Mimo ",

            "header_ver" => " 内核版本 : ",
            "header_mode" => " 运行模式 : ",
            "header_sysproxy" => " 系统代理 : ",
            "header_tun" => " TUN 网卡 : ",
            "header_active_node" => " 代理节点 : ",

            "btn_mode_hint" => " (按 'm' 切换模式)",
            "btn_sysproxy_hint" => " (按 'p' 开关系统代理)",
            "btn_tun_hint" => " (按 'x' 开关 TUN 模式)",
            "btn_restart_hint" => " (按 'r' 重启核心服务)",

            "enabled" => "已开启",
            "disabled" => "已关闭",
            "priv_ok" => " (cap_net_admin: 正常)",
            "priv_req" => " (需要 root/cap_net_admin 权限)",

            "title_overview" => " 核心状态 ",
            "title_throughput" => " 实时速率 ",
            "title_navigation" => " 导航菜单 ",
            "title_proxies_groups" => " 代理分组 ",
            "title_proxies_nodes" => " 节点列表 ",
            "title_profiles_settings" => " 订阅存储 ",
            "title_profiles_list" => " 订阅列表 ",
            "title_connections" => " 活跃连接 ",
            "title_logs" => " 运行日志 ",
            "title_settings" => " 系统设置 ",
            "title_rules" => " 路由规则 ",

            "card_network_services" => " 网络与代理服务 ",
            "card_dns_automation" => " DNS 与自动化 ",
            "card_core_controls" => " 核心服务控制 ",
            "card_ui_preferences" => " 界面与偏好风格 ",
            "card_status_diagnostics" => " 核心与网络出口 ",
            "card_running_metrics" => " 运行指标 ",

            "footer_nav" => " 页面",
            "footer_scroll" => " 滚动",
            "footer_select" => " 选择",
            "footer_test" => " 测速",
            "footer_mode" => " 模式",
            "footer_sysproxy" => " 代理",
            "footer_tun" => " TUN",
            "footer_restart" => " 重启",
            "footer_help" => " 帮助",
            "footer_quit" => " 退出",

            "settings_lang" => "界面语言 (Language)",
            "settings_api_url" => "Mihomo REST API 地址",
            "settings_secret" => "Mihomo API 密钥 (Secret Token)",
            "settings_refresh" => "界面刷新间隔 (ms)",
            "settings_http_port" => "系统代理 HTTP 端口",
            "settings_socks_port" => "系统代理 SOCKS 端口",
            "settings_test_url" => "延迟测试 URL",
            "settings_dns_mode" => "DNS 解析模式 (fake-ip / redir-host)",
            "settings_auto_sysproxy" => "启动时自动开代理 (ON / OFF)",
            "settings_sub_update" => "订阅自动更新周期 (小时)",
            "settings_ui_theme" => "界面配色主题 Preset",
            "settings_save_btn" => " [按 Enter 保存配置到 ~/.config/mimo/config.toml] ",

            _ => key,
        },
        Language::En => match key {
            "tab_dashboard" => " Dashboard ",
            "tab_proxies" => " Proxies ",
            "tab_profiles" => " Profiles ",
            "tab_rules" => " Rules ",
            "tab_connections" => " Connections ",
            "tab_traffic" => " Traffic ",
            "tab_logs" => " Logs ",
            "tab_settings" => " Settings ",
            "tab_privileges" => " Privileges ",
            "tab_about" => " About ",

            "header_ver" => " Core Ver : ",
            "header_mode" => " Mode     : ",
            "header_sysproxy" => " SysProxy : ",
            "header_tun" => " TUN      : ",
            "header_active_node" => " Node     : ",

            "btn_mode_hint" => " (Press 'm' to cycle mode)",
            "btn_sysproxy_hint" => " (Press 'p' to toggle)",
            "btn_tun_hint" => " (Press 'x' to toggle)",
            "btn_restart_hint" => " (Press 'r' to restart)",

            "enabled" => "ENABLED",
            "disabled" => "DISABLED",
            "priv_ok" => " (cap_net_admin: OK)",
            "priv_req" => " (Requires root or cap_net_admin)",

            "title_overview" => " Core Overview ",
            "title_throughput" => " Realtime Speed ",
            "title_navigation" => " Navigation ",
            "title_proxies_groups" => " Proxy Groups ",
            "title_proxies_nodes" => " Node List ",
            "title_profiles_settings" => " Storage Settings ",
            "title_profiles_list" => " Profiles List ",
            "title_connections" => " Active Connections ",
            "title_logs" => " Core Logs ",
            "title_settings" => " System Settings ",
            "title_rules" => " Routing Rules ",

            "card_network_services" => " Network & Ports ",
            "card_dns_automation" => " DNS & Automation ",
            "card_core_controls" => " Core & System Controls ",
            "card_ui_preferences" => " Display & Preferences ",
            "card_status_diagnostics" => " Core & Outbound ",
            "card_running_metrics" => " Metrics ",


            "footer_nav" => " Tabs",
            "footer_scroll" => " Nav",
            "footer_select" => " Select",
            "footer_test" => " Test",
            "footer_mode" => " Mode",
            "footer_sysproxy" => " Proxy",
            "footer_tun" => " TUN",
            "footer_restart" => " Restart",
            "footer_help" => " Help",
            "footer_quit" => " Quit",

            "settings_lang" => "Interface Language",
            "settings_api_url" => "Mihomo REST API URL",
            "settings_secret" => "Mihomo Secret Token",
            "settings_refresh" => "UI Refresh Interval (ms)",
            "settings_http_port" => "System Proxy HTTP Port",
            "settings_socks_port" => "System Proxy SOCKS Port",
            "settings_test_url" => "Latency Test URL",
            "settings_dns_mode" => "DNS Enhanced Mode",
            "settings_auto_sysproxy" => "Launch Auto SysProxy",
            "settings_sub_update" => "Sub Auto-Update Interval (Hours)",
            "settings_ui_theme" => "UI Theme Preset",
            "settings_save_btn" => " [Press Enter to Save to ~/.config/mimo/config.toml] ",

            _ => key,
        },
    }
}

