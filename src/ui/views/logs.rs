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

    // Log Filter Pills & Controls
    let levels = ["all", "info", "warn", "error", "debug"];
    let mut filter_spans = Vec::new();

    let label_filter = match lang { Language::Zh => " 日志过滤: ", Language::En => " Filter: " };
    filter_spans.push(Span::styled(label_filter, Style::default().fg(Theme::TEXT_MUTED)));

    for lvl in &levels {
        let is_sel = state.log_filter.eq_ignore_ascii_case(lvl);
        let style = if is_sel {
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::PRIMARY).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::TEXT_MUTED).bg(Theme::BG_SURFACE)
        };
        filter_spans.push(Span::styled(format!(" {} ", lvl.to_uppercase()), style));
        filter_spans.push(Span::raw(" "));
    }

    let auto_scroll_label = match (state.logs_auto_scroll, lang) {
        (true, Language::Zh) => " [ a: 自动滚动: 开启 ] ",
        (true, Language::En) => " [ a: Auto-Scroll: ON ] ",
        (false, Language::Zh) => " [ a: 自动滚动: 暂停 ] ",
        (false, Language::En) => " [ a: Auto-Scroll: OFF ] ",
    };
    let auto_scroll_style = if state.logs_auto_scroll {
        Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::TEXT_DIM)
    };
    filter_spans.push(Span::styled(auto_scroll_label, auto_scroll_style));
    filter_spans.push(Span::raw(" "));

    let btn_clear_str = match lang { Language::Zh => " [ c: 清空 ] ", Language::En => " [ c: Clear ] " };
    filter_spans.push(Span::styled(btn_clear_str, Style::default().fg(Theme::DANGER_RED).add_modifier(Modifier::BOLD)));

    let title_filter = match lang { Language::Zh => " 日志过滤与控制 ", Language::En => " Log Controls " };
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
            "info" => (Theme::ACTIVE_GREEN, "INFO"),
            "warning" | "warn" => (Theme::WARN_YELLOW, "WARN"),
            "error" => (Theme::DANGER_RED, "ERR "),
            "debug" => (Theme::SECONDARY, "DBG "),
            _ => (Theme::TEXT_MUTED, "LOG "),
        };

        lines.push(Line::from(vec![
            Span::styled(format!("[{}] ", label), Style::default().fg(type_color).add_modifier(Modifier::BOLD)),
            Span::styled(&log.payload, Style::default().fg(Theme::TEXT_MAIN)),
        ]));
    }

    if lines.is_empty() {
        let no_logs_str = match lang { Language::Zh => " 无符合条件的日志记录...", Language::En => " No log entries found..." };
        lines.push(Line::from(Span::styled(no_logs_str, Style::default().fg(Theme::TEXT_MUTED))));
    }

    let title_stream = match lang {
        Language::Zh => format!(" 核心运行日志 ({}) [j/k: 滚动] ", state.logs.len()),
        Language::En => format!(" Core Logs ({}) [j/k: Scroll] ", state.logs.len()),
    };

    let border_style = if state.focus_zone == crate::app::state::FocusZone::Workspace {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let logs_widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title(title_stream),
        )
        .scroll((state.log_scroll as u16, 0))
        .wrap(Wrap { trim: false });

    f.render_widget(logs_widget, chunks[1]);
}
