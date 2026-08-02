use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Log Filter Pills
    let levels = ["all", "info", "warn", "error", "debug"];
    let mut filter_spans = Vec::new();

    let label_filter = match lang { Language::Zh => " 日志过滤: ", Language::En => " Filter: " };
    filter_spans.push(Span::styled(label_filter, Style::default().fg(Theme::TEXT_MUTED)));

    for lvl in &levels {
        let is_sel = state.log_filter.eq_ignore_ascii_case(lvl);
        let style = if is_sel {
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 46))
        };
        filter_spans.push(Span::styled(format!(" {} ", lvl.to_uppercase()), style));
        filter_spans.push(Span::raw(" "));
    }

    let btn_clear_str = match lang { Language::Zh => " [ 🧹 清除日志 (c) ] ", Language::En => " [ 🧹 Clear Logs (c) ] " };
    filter_spans.push(Span::styled(btn_clear_str, Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD)));

    let title_filter = match lang { Language::Zh => " 日志过滤器 Log Filter ", Language::En => " Log Stream Filter " };
    let filter_block = Paragraph::new(Line::from(filter_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_filter),
    );
    f.render_widget(filter_block, chunks[0]);

    let mut lines = Vec::new();

    for log in &state.logs {
        let lvl_str = log.log_type.to_lowercase();
        if state.log_filter != "all" && !lvl_str.contains(&state.log_filter) {
            continue;
        }

        let (type_color, label) = match lvl_str.as_str() {
            "info" => (Color::Rgb(166, 227, 161), "INFO "),
            "warning" | "warn" => (Color::Rgb(249, 226, 175), "WARN "),
            "error" => (Color::Rgb(243, 139, 168), "ERR  "),
            "debug" => (Color::Rgb(137, 220, 235), "DBG  "),
            _ => (Theme::TEXT_MUTED, "LOG  "),
        };

        lines.push(Line::from(vec![
            Span::styled(format!("[{}] ", label), Style::default().fg(type_color).add_modifier(Modifier::BOLD)),
            Span::raw(&log.payload),
        ]));
    }

    if lines.is_empty() {
        let no_logs_str = match lang { Language::Zh => "无符合条件的日志记录...", Language::En => "No log entries found..." };
        lines.push(Line::from(Span::styled(no_logs_str, Style::default().fg(Theme::TEXT_MUTED))));
    }

    let title_stream = match lang {
        Language::Zh => format!(" Mihomo Core 实时日志 Stream ({}) [j/k: 滚动] ", state.logs.len()),
        Language::En => format!(" Mihomo Core Logs Stream ({}) [j/k: Scroll] ", state.logs.len()),
    };

    let logs_widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(title_stream),
        )
        .scroll((state.log_scroll as u16, 0))
        .wrap(Wrap { trim: false });

    f.render_widget(logs_widget, chunks[1]);
}
