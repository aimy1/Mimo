use crate::app::{AppState, Tab};
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let is_focused = state.focus_zone == crate::app::state::FocusZone::Sidebar;
    let border_style = if is_focused {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    // 1. Single unified outer container block for the whole sidebar
    let sidebar_title = match lang {
        Language::Zh => " ⚡ MIMO ",
        Language::En => " ⚡ MIMO ",
    };

    let container_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(Span::styled(
            sidebar_title,
            Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
        ));

    let inner_area = container_block.inner(area);
    f.render_widget(container_block, area);

    // Inner layout: Logo/Version (1 line), Nav List (Min 0), Status info (3 lines)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Version subtitle
            Constraint::Length(1), // Divider spacing
            Constraint::Min(0),    // Nav list
            Constraint::Length(4), // Compact status info
        ])
        .split(inner_area);

    // 1. Version subtitle
    let ver_line = Line::from(vec![
        Span::styled(" v", Style::default().fg(Theme::TEXT_DIM)),
        Span::styled(env!("CARGO_PKG_VERSION"), Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(" (Rust)", Style::default().fg(Theme::TEXT_DIM)),
    ]);
    f.render_widget(Paragraph::new(ver_line).alignment(ratatui::layout::Alignment::Center), chunks[0]);

    // 2. Navigation List
    let items: Vec<ListItem> = Tab::ALL
        .iter()
        .enumerate()
        .map(|(idx, tab)| {
            let is_selected = state.active_tab == *tab;
            let title = tab.title(lang).trim();

            let shortcut_char = if idx < 9 {
                format!("{}", idx + 1)
            } else {
                "0".to_string()
            };

            if is_selected {
                let line = Line::from(vec![
                    Span::styled("▶ ", Style::default().fg(Color::Rgb(17, 17, 27))),
                    Span::styled(format!("{} ", shortcut_char), Style::default().fg(Color::Rgb(17, 17, 27)).add_modifier(Modifier::BOLD)),
                    Span::styled(title, Style::default().fg(Color::Rgb(17, 17, 27)).add_modifier(Modifier::BOLD)),
                ]);
                ListItem::new(line).style(Theme::SIDEBAR_SELECTED)
            } else {
                let line = Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(format!("{} ", shortcut_char), Style::default().fg(Theme::TEXT_DIM)),
                    Span::styled(title, Style::default().fg(Theme::TEXT_MAIN)),
                ]);
                ListItem::new(line)
            }
        })
        .collect();

    let sidebar_list = List::new(items);
    let mut state_list = ListState::default();
    state_list.select(Some(state.active_tab as usize));
    f.render_stateful_widget(sidebar_list, chunks[2], &mut state_list);

    // 3. Compact Status Info at bottom of sidebar
    let active_profile = state
        .profiles
        .iter()
        .find(|p| p.is_active)
        .map(|p| p.name.as_str())
        .unwrap_or("Default");

    let checking_str = match lang {
        Language::Zh => "检测中...",
        Language::En => "Checking...",
    };

    let status_lines = vec![
        Line::from(vec![
            Span::styled(" ──────────────", Style::default().fg(Theme::BORDER_SUBTLE)),
        ]),
        Line::from(vec![
            Span::styled(" 📁 ", Style::default().fg(Theme::SECONDARY)),
            Span::styled(active_profile, Style::default().fg(Theme::TEXT_MAIN)),
        ]),
        Line::from(vec![
            Span::styled(" 🌐 ", Style::default().fg(Theme::WARN_YELLOW)),
            Span::styled(
                state.outbound_ip.as_deref().unwrap_or(checking_str),
                Style::default().fg(Theme::TEXT_SUB),
            ),
        ]),
    ];

    f.render_widget(Paragraph::new(status_lines), chunks[3]);
}

