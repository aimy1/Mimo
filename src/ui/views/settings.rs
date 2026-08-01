use crate::app::AppState;
use crate::ui::i18n::{t, Language};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // 0: Language
            Constraint::Length(3),  // 1: API URL
            Constraint::Length(3),  // 2: API Secret
            Constraint::Length(3),  // 3: HTTP Port
            Constraint::Length(3),  // 4: SOCKS Port
            Constraint::Length(3),  // 5: Latency Test URL
            Constraint::Length(3),  // 6: Refresh Interval
            Constraint::Length(3),  // 7: TUN Network Stack
            Constraint::Length(3),  // 8: Log Level
            Constraint::Length(3),  // 9: Allow LAN
            Constraint::Length(3),  // 10: IPv6 Traffic
            Constraint::Length(3),  // 11: Save Button
            Constraint::Min(0),
        ])
        .split(area);

    // 0: Language
    let style_0 = if state.settings_focus == 0 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let lang_display = if state.settings_lang == "zh" { " 简体中文 (Chinese)  [Space 切换]" } else { " English  [Space Switch]" };
    let lang_block = Paragraph::new(lang_display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_0)
                .title(format!(" {} ", t("settings_lang", lang))),
        );
    f.render_widget(lang_block, chunks[0]);

    // 1: API URL
    let style_1 = if state.settings_focus == 1 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let api_text = if state.settings_focus == 1 {
        format!(" {}_", state.settings_api_url)
    } else {
        format!(" {}", state.settings_api_url)
    };
    let api_block = Paragraph::new(api_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_1)
                .title(format!(" {} [可编辑] ", t("settings_api_url", lang))),
        );
    f.render_widget(api_block, chunks[1]);

    // 2: API Secret
    let style_2 = if state.settings_focus == 2 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let secret_display = if state.settings_focus == 2 {
        format!(" {}_", state.settings_secret)
    } else if state.settings_secret.is_empty() {
        " None (未设置)".to_string()
    } else {
        format!(" {}", state.settings_secret)
    };
    let secret_block = Paragraph::new(secret_display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_2)
                .title(format!(" {} [可编辑] ", t("settings_secret", lang))),
        );
    f.render_widget(secret_block, chunks[2]);

    // 3: HTTP Proxy Port
    let style_3 = if state.settings_focus == 3 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let http_port_text = if state.settings_focus == 3 {
        format!(" {}_", state.settings_http_port)
    } else {
        format!(" {}", state.settings_http_port)
    };
    let http_port_block = Paragraph::new(http_port_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_3)
                .title(" HTTP System Proxy Port [数字编辑] "),
        );
    f.render_widget(http_port_block, chunks[3]);

    // 4: SOCKS Proxy Port
    let style_4 = if state.settings_focus == 4 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let socks_port_text = if state.settings_focus == 4 {
        format!(" {}_", state.settings_socks_port)
    } else {
        format!(" {}", state.settings_socks_port)
    };
    let socks_port_block = Paragraph::new(socks_port_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_4)
                .title(" SOCKS System Proxy Port [数字编辑] "),
        );
    f.render_widget(socks_port_block, chunks[4]);

    // 5: Latency Test URL
    let style_5 = if state.settings_focus == 5 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let test_url_text = if state.settings_focus == 5 {
        format!(" {}_", state.settings_test_url)
    } else {
        format!(" {}", state.settings_test_url)
    };
    let test_url_block = Paragraph::new(test_url_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_5)
                .title(format!(" {} [可编辑] ", t("settings_test_url", lang))),
        );
    f.render_widget(test_url_block, chunks[5]);

    // 6: Refresh Interval
    let style_6 = if state.settings_focus == 6 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let refresh_block = Paragraph::new(format!(" {} ms  [Space 切换 500 / 1000 / 2000 ms]", state.settings_refresh_ms))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_6)
                .title(format!(" {} ", t("settings_refresh", lang))),
        );
    f.render_widget(refresh_block, chunks[6]);

    // 7: TUN Network Stack
    let style_7 = if state.settings_focus == 7 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let stack_block = Paragraph::new(format!(" {}  [Space 切换 system / gvisor / lwip]", state.settings_tun_stack))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_7)
                .title(" TUN Network Stack (网络协议栈) "),
        );
    f.render_widget(stack_block, chunks[7]);

    // 8: Log Level
    let style_8 = if state.settings_focus == 8 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let log_level_block = Paragraph::new(format!(" {}  [Space 切换 info / warning / error / debug / silent]", state.settings_log_level))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_8)
                .title(" Log Level (日志等级) "),
        );
    f.render_widget(log_level_block, chunks[8]);

    // 9: Allow LAN
    let style_9 = if state.settings_focus == 9 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let lan_display = if state.settings_allow_lan { " ● 开启 ON  [Space 切换]" } else { " ○ 关闭 OFF  [Space 切换]" };
    let lan_block = Paragraph::new(lan_display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_9)
                .title(" Allow LAN (允许局域网设备连接) "),
        );
    f.render_widget(lan_block, chunks[9]);

    // 10: IPv6 Traffic
    let style_10 = if state.settings_focus == 10 { Style::default().fg(Theme::BORDER_FOCUS) } else { Style::default().fg(Theme::BORDER) };
    let ipv6_display = if state.settings_ipv6 { " ● 开启 ON  [Space 切换]" } else { " ○ 关闭 OFF  [Space 切换]" };
    let ipv6_block = Paragraph::new(ipv6_display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_10)
                .title(" IPv6 Traffic Support (IPv6 流量接管) "),
        );
    f.render_widget(ipv6_block, chunks[10]);

    // 11: Save Button
    let save_btn_style = if state.settings_focus == 11 {
        Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    };

    let save_block = Paragraph::new(t("settings_save_btn", lang))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Theme::ACTIVE_GREEN)),
        )
        .style(save_btn_style)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(save_block, chunks[11]);
}
