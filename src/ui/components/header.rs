use crate::app::AppState;
use crate::ui::theme::{format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30), // Mode Switcher Pill
            Constraint::Length(25), // System Proxy Pill
            Constraint::Length(20), // TUN Pill
            Constraint::Min(0),    // Speed Readout
        ])
        .split(area);

    let mode_str = state
        .config
        .as_ref()
        .and_then(|c| c.mode.clone())
        .unwrap_or_else(|| "Rule".into());

    let mode_pill = vec![Line::from(vec![
        Span::styled(" 模式 Mode: ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(format!(" {} ", mode_str), Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::MODE_BADGE).add_modifier(Modifier::BOLD)),
        Span::raw(" (m)"),
    ])];
    let mode_block = Paragraph::new(mode_pill).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER)),
    );
    f.render_widget(mode_block, chunks[0]);

    let sys_proxy_style = if state.is_sysproxy_enabled {
        Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::TEXT_MUTED)
    };
    let sys_proxy_str = if state.is_sysproxy_enabled { " ON 已开启 " } else { " OFF 已关闭 " };
    let sys_proxy_pill = vec![Line::from(vec![
        Span::styled(" 系统代理: ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(sys_proxy_str, sys_proxy_style),
        Span::raw(" (p)"),
    ])];
    let sys_proxy_block = Paragraph::new(sys_proxy_pill).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER)),
    );
    f.render_widget(sys_proxy_block, chunks[1]);

    let tun_style = if state.is_tun_enabled {
        Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::TEXT_MUTED)
    };
    let tun_str = if state.is_tun_enabled { " ON " } else { " OFF " };
    let tun_pill = vec![Line::from(vec![
        Span::styled(" TUN: ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(tun_str, tun_style),
        Span::raw(" (x)"),
    ])];
    let tun_block = Paragraph::new(tun_pill).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER)),
    );
    f.render_widget(tun_block, chunks[2]);

    let traffic_text = Line::from(vec![
        Span::styled("▲ ", Style::default().fg(Theme::TRAFFIC_UP).add_modifier(Modifier::BOLD)),
        Span::raw(format_speed(state.current_traffic.up)),
        Span::raw("   "),
        Span::styled("▼ ", Style::default().fg(Theme::TRAFFIC_DOWN).add_modifier(Modifier::BOLD)),
        Span::raw(format_speed(state.current_traffic.down)),
    ]);
    let traffic_widget = Paragraph::new(traffic_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER)),
        )
        .alignment(ratatui::layout::Alignment::Right);
    f.render_widget(traffic_widget, chunks[3]);
}
