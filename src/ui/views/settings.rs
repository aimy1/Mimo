use crate::app::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Summary Status Banner
            Constraint::Min(0),    // 4 Grouped Cards Grid
            Constraint::Length(3), // Bottom Save Button Bar
        ])
        .split(area);

    // -------------------------------------------------------------------------
    // TOP BANNER: Config Metadata & Core Runtime Status
    // -------------------------------------------------------------------------
    let core_ver = state.version.as_ref().map(|v| v.version.as_str()).unwrap_or("v1.19.29 (Meta)");
    let status_str = format!(
        " ⚙️  MIMO v{}   |   Mihomo Core: {}   |   HTTP: {}   SOCKS: {}   MIXED: {}   |   Config: ~/.config/mimo/config.toml ",
        env!("CARGO_PKG_VERSION"), core_ver, state.settings_http_port, state.settings_socks_port, state.settings_mixed_port
    );
    let top_banner = Paragraph::new(status_str)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(137, 220, 235)))
                .title(Span::styled(" 系统运行状态 & 配置文件路径 ", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD))),
        )
        .style(Style::default().fg(Color::Rgb(205, 214, 244)))
        .alignment(Alignment::Center);
    f.render_widget(top_banner, main_chunks[0]);

    // -------------------------------------------------------------------------
    // MIDDLE GRID: 2 Columns x 2 Cards
    // -------------------------------------------------------------------------
    let grid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left Column (Cards 1 & 2)
            Constraint::Percentage(50), // Right Column (Cards 3 & 4)
        ])
        .split(main_chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(52), // Card 1: Network & Proxy Services (items 0..5)
            Constraint::Percentage(48), // Card 2: DNS & Automation (items 6..10)
        ])
        .split(grid_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(52), // Card 3: Core & System Controls (items 11..15)
            Constraint::Percentage(48), // Card 4: Preferences & Themes (items 16..18)
        ])
        .split(grid_chunks[1]);

    // Helper closure to build a row line
    let make_row = |idx: usize, label: &str, value: &str, is_toggle: bool, is_btn: bool| -> Line<'static> {
        let is_focus = state.settings_focus == idx;
        let prefix = if is_focus { " ▶ " } else { "   " };
        
        let label_style = if is_focus {
            Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(205, 214, 244))
        };

        let val_style = if is_btn {
            if is_focus {
                Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)
            }
        } else if is_toggle {
            if value.contains("ON") || value.contains("开启") {
                Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(147, 153, 178))
            }
        } else if is_focus {
            Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(147, 153, 178))
        };

        Line::from(vec![
            Span::styled(prefix.to_string(), if is_focus { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default() }),
            Span::styled(format!("{:<26}", label), label_style),
            Span::styled(value.to_string(), val_style),
        ])
    };

    // -------------------------------------------------------------------------
    // CARD 1: 🌐 网络与代理端口 Network & Proxy Services (Items 0..5)
    // -------------------------------------------------------------------------
    let card1_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if (0..=5).contains(&state.settings_focus) { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) })
        .title(Span::styled(" 🌐 网络与代理服务 Network & Proxy Services ", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)));

    let secret_val = if state.settings_focus == 1 {
        format!("{}█", state.settings_secret)
    } else if state.settings_secret.is_empty() {
        "None (未设置)".to_string()
    } else {
        state.settings_secret.clone()
    };

    let api_url_val = if state.settings_focus == 0 { format!("{}█", state.settings_api_url) } else { state.settings_api_url.clone() };
    let http_val = if state.settings_focus == 2 { format!("{}█", state.settings_http_port) } else { state.settings_http_port.to_string() };
    let socks_val = if state.settings_focus == 3 { format!("{}█", state.settings_socks_port) } else { state.settings_socks_port.to_string() };
    let mixed_val = if state.settings_focus == 4 { format!("{}█", state.settings_mixed_port) } else { state.settings_mixed_port.to_string() };
    let test_val = if state.settings_focus == 5 { format!("{}█", state.settings_test_url) } else { state.settings_test_url.clone() };

    let card1_lines = vec![
        make_row(0, "REST API Base URL", &api_url_val, false, false),
        make_row(1, "API Secret Token", &secret_val, false, false),
        make_row(2, "HTTP Proxy Port", &http_val, false, false),
        make_row(3, "SOCKS Proxy Port", &socks_val, false, false),
        make_row(4, "Mixed Proxy Port", &mixed_val, false, false),
        make_row(5, "Latency Test URL", &test_val, false, false),
    ];

    let p_card1 = Paragraph::new(card1_lines).block(card1_block);
    f.render_widget(p_card1, left_chunks[0]);

    // -------------------------------------------------------------------------
    // CARD 2: 🛡️ DNS 与高级自动化 DNS & Automation (Items 6..10)
    // -------------------------------------------------------------------------
    let card2_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if (6..=10).contains(&state.settings_focus) { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) })
        .title(Span::styled(" 🛡️ DNS 与自动化服务 DNS & Automation ", Style::default().fg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD)));

    let dns_val = format!("{} [Space 切换]", state.settings_dns_mode);
    let sniff_val = if state.settings_sniffing { "● 开启 ON [Space]" } else { "○ 关闭 OFF [Space]" };
    let tcp_val = if state.settings_tcp_concurrent { "● 开启 ON [Space]" } else { "○ 关闭 OFF [Space]" };
    let auto_sys_val = if state.settings_auto_sysproxy { "● 开启 ON [Space]" } else { "○ 关闭 OFF [Space]" };
    let sub_val = match state.settings_sub_update_hours {
        0 => "手动刷新 Manual [Space]".to_string(),
        h => format!("每 {} 小时自动更新 [Space]", h),
    };

    let card2_lines = vec![
        make_row(6, "DNS Enhanced Mode", &dns_val, false, false),
        make_row(7, "TLS Sniffer 嗅探", sniff_val, true, false),
        make_row(8, "TCP Concurrent 并发", tcp_val, true, false),
        make_row(9, "Launch Auto SysProxy", auto_sys_val, true, false),
        make_row(10, "Sub Auto-Update", &sub_val, false, false),
    ];

    let p_card2 = Paragraph::new(card2_lines).block(card2_block);
    f.render_widget(p_card2, left_chunks[1]);

    // -------------------------------------------------------------------------
    // CARD 3: ⚡ 核心服务与提权 Core & System Controls (Items 11..15)
    // -------------------------------------------------------------------------
    let card3_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if (11..=15).contains(&state.settings_focus) { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) })
        .title(Span::styled(" ⚡ 核心与高级服务 Core & System Controls ", Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)));

    let stack_val = format!("{} [Space 切换]", state.settings_tun_stack);
    let log_val = format!("{} [Space 切换]", state.settings_log_level);
    let lan_val = if state.settings_allow_lan { "● 开启 ON [Space]" } else { "○ 关闭 OFF [Space]" };
    let ipv6_val = if state.settings_ipv6 { "● 开启 ON [Space]" } else { "○ 关闭 OFF [Space]" };
    let dl_core_val = "[ ⬇️ 自动下载 / 更新 Mihomo 核心 (Enter/Space) ]";

    let card3_lines = vec![
        make_row(11, "TUN Network Stack", &stack_val, false, false),
        make_row(12, "Log Level 日志等级", &log_val, false, false),
        make_row(13, "Allow LAN 局域网共享", lan_val, true, false),
        make_row(14, "IPv6 Support 支持", ipv6_val, true, false),
        make_row(15, "Mihomo Core Manager", dl_core_val, false, true),
    ];

    let p_card3 = Paragraph::new(card3_lines).block(card3_block);
    f.render_widget(p_card3, right_chunks[0]);

    // -------------------------------------------------------------------------
    // CARD 4: 🎨 界面与偏好风格 Preferences & Themes (Items 16..18)
    // -------------------------------------------------------------------------
    let card4_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if (16..=18).contains(&state.settings_focus) { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) })
        .title(Span::styled(" 🎨 界面与偏好风格 Preferences & Display ", Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)));

    let lang_val = if state.settings_lang == "zh" { "简体中文 [Space]" } else { "English [Space]" };
    let theme_val = format!("{} [Space]", state.settings_ui_theme);
    let refresh_val = format!("{} ms [Space]", state.settings_refresh_ms);

    let card4_lines = vec![
        make_row(16, "Interface Language", lang_val, false, false),
        make_row(17, "UI Color Theme", &theme_val, false, false),
        make_row(18, "UI Refresh Interval", &refresh_val, false, false),
    ];

    let p_card4 = Paragraph::new(card4_lines).block(card4_block);
    f.render_widget(p_card4, right_chunks[1]);

    // -------------------------------------------------------------------------
    // BOTTOM BAR: Save Button (Focus Item: 19)
    // -------------------------------------------------------------------------
    let save_btn_style = if state.settings_focus == 19 {
        Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    };

    let save_text = if state.settings_focus == 19 {
        " ▶ [ 💾 保存所有系统设置至 ~/.config/mimo/config.toml (按 Enter 保存) ] ◀ "
    } else {
        " [ 💾 保存所有系统设置 (Save All Settings) ] "
    };

    let save_block = Paragraph::new(save_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if state.settings_focus == 19 { Theme::BORDER_FOCUS } else { Theme::ACTIVE_GREEN })),
        )
        .style(save_btn_style)
        .alignment(Alignment::Center);

    f.render_widget(save_block, main_chunks[2]);
}

