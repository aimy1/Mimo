use crate::app::AppState;
use crate::ui::i18n::{t, Language};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // 4 Grouped Cards Grid
            Constraint::Length(3), // Bottom Save Button Bar
        ])
        .split(area);

    let grid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left Column (Cards 1 & 2)
            Constraint::Percentage(50), // Right Column (Cards 3 & 4)
        ])
        .split(main_chunks[0]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55), // Card 1: Network & Proxy Services (items 0..4)
            Constraint::Percentage(45), // Card 2: DNS & Automation Services (items 5..7)
        ])
        .split(grid_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55), // Card 3: Core & System Controls (items 8..11)
            Constraint::Percentage(45), // Card 4: Preferences & Display Themes (items 12..14)
        ])
        .split(grid_chunks[1]);

    // -------------------------------------------------------------------------
    // LEFT COLUMN TOP: Card 1 - 🌐 网络与代理服务 Network & Proxy Services
    // Focus Items: 0 (API URL), 1 (Secret), 2 (HTTP Port), 3 (SOCKS Port), 4 (Test URL)
    // -------------------------------------------------------------------------
    let card1_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" 🌐 网络与代理服务 Network & Proxy Services ", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)));

    let card1_inner = card1_block.inner(left_chunks[0]);
    f.render_widget(card1_block, left_chunks[0]);

    let card1_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // 0: API Base URL
            Constraint::Length(3), // 1: Secret Token
            Constraint::Length(3), // 2: HTTP Port
            Constraint::Length(3), // 3: SOCKS Port
            Constraint::Length(3), // 4: Latency Test URL
            Constraint::Min(0),
        ])
        .split(card1_inner);

    // 0: API Base URL
    let style_0 = if state.settings_focus == 0 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let api_text = if state.settings_focus == 0 { format!(" {}_", state.settings_api_url) } else { format!(" {}", state.settings_api_url) };
    let p_0 = Paragraph::new(api_text).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_0).title(" REST API Base URL "),
    );
    f.render_widget(p_0, card1_chunks[0]);

    // 1: Secret Token
    let style_1 = if state.settings_focus == 1 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let secret_display = if state.settings_focus == 1 {
        format!(" {}_", state.settings_secret)
    } else if state.settings_secret.is_empty() {
        " None (未设置)".to_string()
    } else {
        format!(" {}", state.settings_secret)
    };
    let p_1 = Paragraph::new(secret_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_1).title(" API Secret Token "),
    );
    f.render_widget(p_1, card1_chunks[1]);

    // 2: HTTP Proxy Port
    let style_2 = if state.settings_focus == 2 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let http_port_text = if state.settings_focus == 2 { format!(" {}_", state.settings_http_port) } else { format!(" {}", state.settings_http_port) };
    let p_2 = Paragraph::new(http_port_text).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_2).title(" HTTP System Proxy Port "),
    );
    f.render_widget(p_2, card1_chunks[2]);

    // 3: SOCKS Proxy Port
    let style_3 = if state.settings_focus == 3 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let socks_port_text = if state.settings_focus == 3 { format!(" {}_", state.settings_socks_port) } else { format!(" {}", state.settings_socks_port) };
    let p_3 = Paragraph::new(socks_port_text).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_3).title(" SOCKS System Proxy Port "),
    );
    f.render_widget(p_3, card1_chunks[3]);

    // 4: Latency Test URL
    let style_4 = if state.settings_focus == 4 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let test_url_text = if state.settings_focus == 4 { format!(" {}_", state.settings_test_url) } else { format!(" {}", state.settings_test_url) };
    let p_4 = Paragraph::new(test_url_text).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_4).title(" Latency Test URL (节点测速 URL) "),
    );
    f.render_widget(p_4, card1_chunks[4]);

    // -------------------------------------------------------------------------
    // LEFT COLUMN BOTTOM: Card 2 - 🛡️ DNS 与系统自动化 DNS & Automation
    // Focus Items: 5 (DNS Mode), 6 (Auto SysProxy), 7 (Sub Auto Update Hours)
    // -------------------------------------------------------------------------
    let card2_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" 🛡️ DNS 与自动化服务 DNS & Automation ", Style::default().fg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD)));

    let card2_inner = card2_block.inner(left_chunks[1]);
    f.render_widget(card2_block, left_chunks[1]);

    let card2_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // 5: DNS Enhanced Mode
            Constraint::Length(3), // 6: Auto SysProxy on Launch
            Constraint::Length(3), // 7: Sub Auto-Update Interval
            Constraint::Min(0),
        ])
        .split(card2_inner);

    // 5: DNS Enhanced Mode
    let style_5 = if state.settings_focus == 5 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_5 = Paragraph::new(format!(" {}  [Space 切换 fake-ip / redir-host]", state.settings_dns_mode)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_5).title(" DNS Enhanced Mode (解析模式) "),
    );
    f.render_widget(p_5, card2_chunks[0]);

    // 6: Auto SysProxy on Launch
    let style_6 = if state.settings_focus == 6 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let sysproxy_display = if state.settings_auto_sysproxy { " ● 开启 ON  [Space 切换]" } else { " ○ 关闭 OFF  [Space 切换]" };
    let p_6 = Paragraph::new(sysproxy_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_6).title(" Launch Auto SysProxy (启动自启代理) "),
    );
    f.render_widget(p_6, card2_chunks[1]);

    // 7: Sub Auto-Update Interval
    let style_7 = if state.settings_focus == 7 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let sub_update_str = match state.settings_sub_update_hours {
        0 => " 手动刷新 Manual  [Space 切换]".to_string(),
        h => format!(" 每 {} 小时自动更新  [Space 切换]", h),
    };
    let p_7 = Paragraph::new(sub_update_str).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_7).title(" Sub Auto-Update (订阅自动更新周期) "),
    );
    f.render_widget(p_7, card2_chunks[2]);

    // -------------------------------------------------------------------------
    // RIGHT COLUMN TOP: Card 3 - ⚡ 核心与高级控制 Core & System Controls
    // Focus Items: 8 (TUN Stack), 9 (Log Level), 10 (Allow LAN), 11 (IPv6)
    // -------------------------------------------------------------------------
    let card3_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" ⚡ 核心与高级服务 Core & System Controls ", Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)));

    let card3_inner = card3_block.inner(right_chunks[0]);
    f.render_widget(card3_block, right_chunks[0]);

    let card3_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // 8: TUN Stack
            Constraint::Length(3), // 9: Log Level
            Constraint::Length(3), // 10: Allow LAN
            Constraint::Length(3), // 11: IPv6 Traffic
            Constraint::Length(3), // 12: Download / Update Mihomo Core
            Constraint::Min(0),
        ])
        .split(card3_inner);

    // 8: TUN Stack
    let style_8 = if state.settings_focus == 8 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_8 = Paragraph::new(format!(" {}  [Space 切换 system / gvisor / lwip]", state.settings_tun_stack)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_8).title(" TUN Network Stack (协议栈) "),
    );
    f.render_widget(p_8, card3_chunks[0]);

    // 9: Log Level
    let style_9 = if state.settings_focus == 9 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_9 = Paragraph::new(format!(" {}  [Space 切换 info / warning / error / debug / silent]", state.settings_log_level)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_9).title(" Log Level (日志等级) "),
    );
    f.render_widget(p_9, card3_chunks[1]);

    // 10: Allow LAN
    let style_10 = if state.settings_focus == 10 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let lan_display = if state.settings_allow_lan { " ● 开启 ON  [Space 切换]" } else { " ○ 关闭 OFF  [Space 切换]" };
    let p_10 = Paragraph::new(lan_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_10).title(" Allow LAN (局域网设备共享) "),
    );
    f.render_widget(p_10, card3_chunks[2]);

    // 11: IPv6 Traffic
    let style_11 = if state.settings_focus == 11 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let ipv6_display = if state.settings_ipv6 { " ● 开启 ON  [Space 切换]" } else { " ○ 关闭 OFF  [Space 切换]" };
    let p_11 = Paragraph::new(ipv6_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_11).title(" IPv6 Support (IPv6 流量控制) "),
    );
    f.render_widget(p_11, card3_chunks[3]);

    // 12: Download / Update Mihomo Core Binary
    let style_12 = if state.settings_focus == 12 { Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::Rgb(203, 166, 247)) };
    let p_12 = Paragraph::new(" ⬇️  下载 / 更新 Mihomo 核心 (Download/Update Core)  [Space / Enter 触发]").block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(if state.settings_focus == 12 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) }).title(" Mihomo Binary Manager (核心版本管理) "),
    ).style(style_12);
    f.render_widget(p_12, card3_chunks[4]);

    // -------------------------------------------------------------------------
    // RIGHT COLUMN BOTTOM: Card 4 - 🎨 界面与偏好风格 Preferences & Themes
    // Focus Items: 13 (Language), 14 (UI Theme), 15 (Refresh Rate)
    // -------------------------------------------------------------------------
    let card4_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" 🎨 界面与偏好风格 Preferences & Display ", Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)));

    let card4_inner = card4_block.inner(right_chunks[1]);
    f.render_widget(card4_block, right_chunks[1]);

    let card4_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // 13: Language
            Constraint::Length(3), // 14: UI Theme
            Constraint::Length(3), // 15: Refresh Rate
            Constraint::Min(0),
        ])
        .split(card4_inner);

    // 13: Language
    let style_13 = if state.settings_focus == 13 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let lang_display = if state.settings_lang == "zh" { " 简体中文 (Chinese)  [Space 切换]" } else { " English  [Space Switch]" };
    let p_13 = Paragraph::new(lang_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_13).title(" Interface Language (语言) "),
    );
    f.render_widget(p_13, card4_chunks[0]);

    // 14: UI Theme
    let style_14 = if state.settings_focus == 14 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_14 = Paragraph::new(format!(" {}  [Space 切换 Catppuccin / Nord / TokyoNight / Gruvbox]", state.settings_ui_theme)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_14).title(" UI Color Theme (界面配色主题) "),
    );
    f.render_widget(p_14, card4_chunks[1]);

    // 15: Refresh Rate
    let style_15 = if state.settings_focus == 15 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_15 = Paragraph::new(format!(" {} ms  [Space 切换 500 / 1000 / 2000 ms]", state.settings_refresh_ms)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_15).title(" UI Refresh Interval (刷新频率) "),
    );
    f.render_widget(p_15, card4_chunks[2]);

    // -------------------------------------------------------------------------
    // BOTTOM BAR: Save Button (Focus Item: 16)
    // -------------------------------------------------------------------------
    let save_btn_style = if state.settings_focus == 16 {
        Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    };

    let save_block = Paragraph::new(t("settings_save_btn", lang))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::ACTIVE_GREEN)),
        )
        .style(save_btn_style)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(save_block, main_chunks[1]);
}
