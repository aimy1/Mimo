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
            Constraint::Min(0),    // Grouped Cards Grid
            Constraint::Length(3), // Bottom Save Button Bar
        ])
        .split(area);

    let grid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left Column: Network & Proxy Services
            Constraint::Percentage(50), // Right Column: Core Controls & UI Preferences
        ])
        .split(main_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Top: Core Controls
            Constraint::Percentage(40), // Bottom: UI Preferences
        ])
        .split(grid_chunks[1]);

    // -------------------------------------------------------------------------
    // LEFT COLUMN: Card 1 - 🌐 网络与代理端口 (Network & Proxy Services)
    // Focus Items: 0 (API URL), 1 (Secret), 2 (HTTP Port), 3 (SOCKS Port), 4 (Test URL)
    // -------------------------------------------------------------------------
    let card1_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" 🌐 网络与代理服务 Network & Proxy Services ", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)));

    let card1_inner = card1_block.inner(grid_chunks[0]);
    f.render_widget(card1_block, grid_chunks[0]);

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
    // RIGHT COLUMN TOP: Card 2 - ⚡ 核心与高级控制 (Core & System Features)
    // Focus Items: 5 (TUN Stack), 6 (Log Level), 7 (Allow LAN), 8 (IPv6)
    // -------------------------------------------------------------------------
    let card2_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" ⚡ 核心与高级服务 Core & System Controls ", Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)));

    let card2_inner = card2_block.inner(right_chunks[0]);
    f.render_widget(card2_block, right_chunks[0]);

    let card2_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // 5: TUN Stack
            Constraint::Length(3), // 6: Log Level
            Constraint::Length(3), // 7: Allow LAN
            Constraint::Length(3), // 8: IPv6 Traffic
            Constraint::Min(0),
        ])
        .split(card2_inner);

    // 5: TUN Stack
    let style_5 = if state.settings_focus == 5 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_5 = Paragraph::new(format!(" {}  [Space 切换 system / gvisor / lwip]", state.settings_tun_stack)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_5).title(" TUN Network Stack (协议栈) "),
    );
    f.render_widget(p_5, card2_chunks[0]);

    // 6: Log Level
    let style_6 = if state.settings_focus == 6 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_6 = Paragraph::new(format!(" {}  [Space 切换 info / warning / error / debug / silent]", state.settings_log_level)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_6).title(" Log Level (日志等级) "),
    );
    f.render_widget(p_6, card2_chunks[1]);

    // 7: Allow LAN
    let style_7 = if state.settings_focus == 7 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let lan_display = if state.settings_allow_lan { " ● 开启 ON  [Space 切换]" } else { " ○ 关闭 OFF  [Space 切换]" };
    let p_7 = Paragraph::new(lan_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_7).title(" Allow LAN (局域网设备共享) "),
    );
    f.render_widget(p_7, card2_chunks[2]);

    // 8: IPv6 Traffic
    let style_8 = if state.settings_focus == 8 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let ipv6_display = if state.settings_ipv6 { " ● 开启 ON  [Space 切换]" } else { " ○ 关闭 OFF  [Space 切换]" };
    let p_8 = Paragraph::new(ipv6_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_8).title(" IPv6 Support (IPv6 流量控制) "),
    );
    f.render_widget(p_8, card2_chunks[3]);

    // -------------------------------------------------------------------------
    // RIGHT COLUMN BOTTOM: Card 3 - 🎨 界面与偏好 (UI & Preferences)
    // Focus Items: 9 (Language), 10 (Refresh Rate)
    // -------------------------------------------------------------------------
    let card3_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" 🎨 界面与偏好 Preferences ", Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)));

    let card3_inner = card3_block.inner(right_chunks[1]);
    f.render_widget(card3_block, right_chunks[1]);

    let card3_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // 9: Language
            Constraint::Length(3), // 10: Refresh Rate
            Constraint::Min(0),
        ])
        .split(card3_inner);

    // 9: Language
    let style_9 = if state.settings_focus == 9 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let lang_display = if state.settings_lang == "zh" { " 简体中文 (Chinese)  [Space 切换]" } else { " English  [Space Switch]" };
    let p_9 = Paragraph::new(lang_display).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_9).title(" Interface Language (语言) "),
    );
    f.render_widget(p_9, card3_chunks[0]);

    // 10: Refresh Rate
    let style_10 = if state.settings_focus == 10 { Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD) } else { Style::default().fg(Theme::BORDER) };
    let p_10 = Paragraph::new(format!(" {} ms  [Space 切换 500 / 1000 / 2000 ms]", state.settings_refresh_ms)).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(style_10).title(" UI Refresh Interval (刷新频率) "),
    );
    f.render_widget(p_10, card3_chunks[1]);

    // -------------------------------------------------------------------------
    // BOTTOM BAR: Save Button (Focus Item: 11)
    // -------------------------------------------------------------------------
    let save_btn_style = if state.settings_focus == 11 {
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
