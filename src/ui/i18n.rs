use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Language {
    Zh,
    En,
}

impl Default for Language {
    fn default() -> Self {
        Language::Zh
    }
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "en" | "english" => Language::En,
            _ => Language::Zh,
        }
    }
}

pub fn t<'a>(key: &'a str, lang: Language) -> &'a str {
    match lang {
        Language::Zh => match key {
            "tab_dashboard" => " 仪表盘 ",
            "tab_proxies" => " 节点选择 ",
            "tab_profiles" => " 订阅配置 ",
            "tab_rules" => " 规则路由 ",
            "tab_connections" => " 链接管理 ",
            "tab_traffic" => " 流量监控 ",
            "tab_logs" => " 运行日志 ",
            "tab_settings" => " 系统设置 ",
            "tab_privileges" => " 权限管理 ",
            "tab_about" => " 关于 Mimo ",

            "header_ver" => " 内核版本 : ",
            "header_mode" => " 运行模式 : ",
            "header_sysproxy" => " 系统代理 : ",
            "header_tun" => " TUN 虚拟网卡 : ",
            "header_active_node" => " 当前代理节点 : ",

            "btn_mode_hint" => " (按 'm' 切换 规则/全局/直连)",
            "btn_sysproxy_hint" => " (按 'p' 开关系统代理)",
            "btn_tun_hint" => " (按 'x' 开关 TUN 模式)",
            "btn_restart_hint" => " (按 'r' 重启内核服务)",

            "enabled" => "已开启",
            "disabled" => "已关闭",
            "priv_ok" => " (cap_net_admin: 正常)",
            "priv_req" => " (需要 root 或 cap_net_admin 权限)",

            "title_overview" => " Mihomo 核心状态 ",
            "title_throughput" => " 实时网络速率 ",
            "title_navigation" => " 快速导航 ",
            "title_proxies_groups" => " 代理分组 [支持鼠标点击 / 左右键切换] ",
            "title_proxies_nodes" => " 节点列表 [回车/点击选择 | 'd'测速 | 's'搜索] ",
            "title_profiles_settings" => " 订阅配置目录 ",
            "title_profiles_list" => " 本地订阅列表 ",
            "title_connections" => " 活跃连接列表 ['d'/点击断开] ",
            "title_logs" => " 实时内核日志 ",
            "title_settings" => " Mimo 系统设置 ",
            "title_rules" => " 路由规则过滤列表 ",

            "card_network_services" => " 🌐 网络与代理服务 Network & Proxy Services ",
            "card_dns_automation" => " 🛡️ DNS 与自动化服务 DNS & Automation ",
            "card_core_controls" => " ⚡ 核心与高级服务 Core & System Controls ",
            "card_ui_preferences" => " 🎨 界面与偏好风格 Preferences & Display ",
            "card_status_diagnostics" => " 🌐 核心与网络出口诊断 Status & Outbound ",
            "card_running_metrics" => " 📊 运行概览 Running Metrics ",

            "footer_nav" => " 切换页面",
            "footer_scroll" => " 导航/滚动",
            "footer_select" => " 选择",
            "footer_test" => " 测速",
            "footer_mode" => " 模式",
            "footer_sysproxy" => " 系统代理",
            "footer_tun" => " TUN模式",
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
            "settings_save_btn" => " [按 Enter / 点击保存配置到 ~/.config/mimo/config.toml] ",

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

            "header_ver" => " Core Version : ",
            "header_mode" => " Operation Mode : ",
            "header_sysproxy" => " System Proxy  : ",
            "header_tun" => " TUN Virtual Adapter : ",
            "header_active_node" => " Active Proxy Node : ",

            "btn_mode_hint" => " (Press 'm' to cycle Rule/Global/Direct)",
            "btn_sysproxy_hint" => " (Press 'p' to toggle)",
            "btn_tun_hint" => " (Press 'x' to toggle)",
            "btn_restart_hint" => " (Press 'r' to restart Core)",

            "enabled" => "ENABLED",
            "disabled" => "DISABLED",
            "priv_ok" => " (cap_net_admin: OK)",
            "priv_req" => " (Requires root or cap_net_admin)",

            "title_overview" => " Mihomo Core Overview ",
            "title_throughput" => " Realtime Speed ",
            "title_navigation" => " Quick Navigation ",
            "title_proxies_groups" => " Proxy Groups [Mouse Click / Left-Right focus] ",
            "title_proxies_nodes" => " Node List [Enter/Click Select | 'd' Test | 's' Search] ",
            "title_profiles_settings" => " Profile Storage Settings ",
            "title_profiles_list" => " Profiles List ",
            "title_connections" => " Active Connections ['d'/Click Close] ",
            "title_logs" => " Mihomo Logs Stream ",
            "title_settings" => " Mimo Settings ",
            "title_rules" => " Routing Rules List ",

            "card_network_services" => " 🌐 Network & Proxy Services ",
            "card_dns_automation" => " 🛡️ DNS & Automation ",
            "card_core_controls" => " ⚡ Core & System Controls ",
            "card_ui_preferences" => " 🎨 Preferences & Display ",
            "card_status_diagnostics" => " 🌐 Core & Outbound Diagnostics ",
            "card_running_metrics" => " 📊 Running Metrics ",

            "footer_nav" => " Switch View",
            "footer_scroll" => " Nav/Scroll",
            "footer_select" => " Select",
            "footer_test" => " Test",
            "footer_mode" => " Mode",
            "footer_sysproxy" => " SysProxy",
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
            "settings_save_btn" => " [Press Enter / Click to Save to ~/.config/mimo/config.toml] ",

            _ => key,
        },
    }
}
