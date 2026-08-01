use crate::app::AppState;
use crate::ui::i18n::{t, Language};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 0: Language
            Constraint::Length(3), // 1: API URL
            Constraint::Length(3), // 2: API Secret
            Constraint::Length(3), // 3: Refresh Interval
            Constraint::Length(3), // 4: HTTP Port
            Constraint::Length(3), // 5: SOCKS Port
            Constraint::Length(3), // 6: Latency Test URL
            Constraint::Length(3), // 7: Save Button
            Constraint::Min(0),
        ])
        .split(area);

    // 1. Language Focus: 0
    let style_0 = if state.settings_focus == 0 { Style::default().fg(Color::Cyan) } else { Style::default().fg(Theme::BORDER) };
    let lang_display = if state.settings_lang == "zh" { "简体中文 (Chinese)  [按 Space / 点击切换]" } else { "English  [Press Space / Click to Switch]" };
    let lang_block = Paragraph::new(Line::from(vec![
        Span::styled(format!("  {} : ", t("settings_lang", lang)), Style::default().fg(Color::DarkGray)),
        Span::styled(lang_display, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(style_0)
            .title(format!(" {} ", t("settings_lang", lang))),
    );
    f.render_widget(lang_block, chunks[0]);

    // 2. API URL Focus: 1
    let style_1 = if state.settings_focus == 1 { Style::default().fg(Color::Cyan) } else { Style::default().fg(Theme::BORDER) };
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
                .title(format!(" {} [键盘可编辑] ", t("settings_api_url", lang))),
        );
    f.render_widget(api_block, chunks[1]);

    // 3. API Secret Focus: 2
    let style_2 = if state.settings_focus == 2 { Style::default().fg(Color::Cyan) } else { Style::default().fg(Theme::BORDER) };
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
                .title(format!(" {} [键盘可编辑] ", t("settings_secret", lang))),
        );
    f.render_widget(secret_block, chunks[2]);

    // 4. Refresh Interval Focus: 3
    let style_3 = if state.settings_focus == 3 { Style::default().fg(Color::Cyan) } else { Style::default().fg(Theme::BORDER) };
    let refresh_block = Paragraph::new(format!(" {} ms  [按 Space 切换 500 / 1000 / 2000 ms]", state.settings_refresh_ms))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_3)
                .title(format!(" {} ", t("settings_refresh", lang))),
        );
    f.render_widget(refresh_block, chunks[3]);

    // 5. HTTP Proxy Port Focus: 4
    let style_4 = if state.settings_focus == 4 { Style::default().fg(Color::Cyan) } else { Style::default().fg(Theme::BORDER) };
    let http_port_text = if state.settings_focus == 4 {
        format!(" {}_", state.settings_http_port)
    } else {
        format!(" {}", state.settings_http_port)
    };
    let http_port_block = Paragraph::new(http_port_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_4)
                .title(" HTTP System Proxy Port [数字输入] "),
        );
    f.render_widget(http_port_block, chunks[4]);

    // 6. SOCKS Proxy Port Focus: 5
    let style_5 = if state.settings_focus == 5 { Style::default().fg(Color::Cyan) } else { Style::default().fg(Theme::BORDER) };
    let socks_port_text = if state.settings_focus == 5 {
        format!(" {}_", state.settings_socks_port)
    } else {
        format!(" {}", state.settings_socks_port)
    };
    let socks_port_block = Paragraph::new(socks_port_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_5)
                .title(" SOCKS System Proxy Port [数字输入] "),
        );
    f.render_widget(socks_port_block, chunks[5]);

    // 7. Latency Test URL Focus: 6
    let style_6 = if state.settings_focus == 6 { Style::default().fg(Color::Cyan) } else { Style::default().fg(Theme::BORDER) };
    let test_url_text = if state.settings_focus == 6 {
        format!(" {}_", state.settings_test_url)
    } else {
        format!(" {}", state.settings_test_url)
    };
    let test_url_block = Paragraph::new(test_url_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(style_6)
                .title(format!(" {} [键盘可编辑] ", t("settings_test_url", lang))),
        );
    f.render_widget(test_url_block, chunks[6]);

    // 8. Save Button Focus: 7
    let save_btn_style = if state.settings_focus == 7 {
        Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    };

    let save_block = Paragraph::new(t("settings_save_btn", lang))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green)),
        )
        .style(save_btn_style)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(save_block, chunks[7]);
}
