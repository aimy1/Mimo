use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::{format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    // 1. Single unified outer block for the entire header
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER));
    
    let inner_area = header_block.inner(area);
    f.render_widget(header_block, area);

    // Split inner area into left pills and right traffic
    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),      // Control Pills
            Constraint::Length(30),  // Traffic Readout
        ])
        .split(inner_area);

    let (label_mode, label_sys, label_tun) = match lang {
        Language::Zh => ("模式", "系统代理", "TUN模式"),
        Language::En => ("Mode", "SysProxy", "TUN"),
    };

    // Mode Pill
    let mode_str = state
        .config
        .as_ref()
        .and_then(|c| c.mode.clone())
        .unwrap_or_else(|| "Rule".into());

    let sys_proxy_style = if state.is_sysproxy_enabled {
        Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::TEXT_MUTED).bg(Theme::BG_SURFACE)
    };
    let sys_proxy_text = if state.is_sysproxy_enabled { " ON " } else { " OFF " };

    let tun_style = if state.is_tun_enabled {
        Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::TEXT_MUTED).bg(Theme::BG_SURFACE)
    };
    let tun_text = if state.is_tun_enabled { " ON " } else { " OFF " };

    let left_spans = vec![
        Span::raw(" "),
        Span::styled(format!("{}: ", label_mode), Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(format!(" {} ", mode_str), Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::MODE_BADGE).add_modifier(Modifier::BOLD)),
        Span::styled(" [m]", Style::default().fg(Theme::TEXT_DIM)),
        Span::raw("   "),
        Span::styled(format!("{}: ", label_sys), Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(sys_proxy_text, sys_proxy_style),
        Span::styled(" [p]", Style::default().fg(Theme::TEXT_DIM)),
        Span::raw("   "),
        Span::styled(format!("{}: ", label_tun), Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(tun_text, tun_style),
        Span::styled(" [x]", Style::default().fg(Theme::TEXT_DIM)),
    ];

    let left_widget = Paragraph::new(Line::from(left_spans));
    f.render_widget(left_widget, inner_chunks[0]);

    // Right Traffic Speed Spans
    let traffic_spans = vec![
        Span::styled("▲ ", Style::default().fg(Theme::TRAFFIC_UP).add_modifier(Modifier::BOLD)),
        Span::styled(format!("{:<9}", format_speed(state.current_traffic.up)), Style::default().fg(Theme::TEXT_MAIN)),
        Span::raw(" "),
        Span::styled("▼ ", Style::default().fg(Theme::TRAFFIC_DOWN).add_modifier(Modifier::BOLD)),
        Span::styled(format!("{:<9}", format_speed(state.current_traffic.down)), Style::default().fg(Theme::TEXT_MAIN)),
    ];

    let traffic_widget = Paragraph::new(Line::from(traffic_spans))
        .alignment(ratatui::layout::Alignment::Right);
    f.render_widget(traffic_widget, inner_chunks[1]);
}

