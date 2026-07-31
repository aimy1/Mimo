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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Logo Header
            Constraint::Min(0),    // Navigation List
            Constraint::Length(4), // Bottom Active Profile Status Card
        ])
        .split(area);

    // 1. Top Logo Banner with Rounded Border
    let logo_text = vec![Line::from(vec![
        Span::styled("⚡ MIMO ", Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)),
        Span::styled("v0.1.0", Style::default().fg(Theme::TEXT_MUTED)),
    ])];
    let logo_block = Paragraph::new(logo_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER_FOCUS)),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(logo_block, chunks[0]);

    // 2. Navigation List
    let icons = [
        "📊 ", // Dashboard
        "⚡ ", // Proxies
        "📁 ", // Profiles
        "📜 ", // Rules
        "🔗 ", // Connections
        "📈 ", // Traffic
        "📝 ", // Logs
        "⚙️ ", // Settings
    ];

    let items: Vec<ListItem> = Tab::ALL
        .iter()
        .enumerate()
        .map(|(idx, tab)| {
            let is_selected = state.active_tab == *tab;
            let icon = icons.get(idx).unwrap_or(&"  ");
            let title = tab.title(lang);
            let prefix = if is_selected { "▶ " } else { "  " };

            let style = if is_selected {
                Theme::SIDEBAR_SELECTED
            } else {
                Style::default().fg(Color::White)
            };

            let line = Line::from(vec![
                Span::styled(prefix, if is_selected { Style::default().fg(Color::Rgb(17, 17, 27)) } else { Style::default().fg(Theme::BORDER_FOCUS) }),
                Span::styled(format!("{} ", icon), style),
                Span::styled(format!("{:<8}", title.trim()), style),
            ]);

            ListItem::new(line).style(style)
        })
        .collect();

    let sidebar_border_style = if state.focus_zone == crate::app::state::FocusZone::Sidebar {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let sidebar_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(sidebar_border_style)
                .title(Span::styled(" 菜单 Menu ", Style::default().fg(Theme::TEXT_MUTED))),
        );

    let mut state_list = ListState::default();
    state_list.select(Some(state.active_tab as usize));
    f.render_stateful_widget(sidebar_list, chunks[1], &mut state_list);

    // 3. Bottom Active Profile Status Card
    let active_profile = state
        .profiles
        .iter()
        .find(|p| p.is_active)
        .map(|p| p.name.as_str())
        .unwrap_or("Default");

    let card_text = vec![
        Line::from(vec![
            Span::styled("配置: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(active_profile, Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("模式: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(
                state.config.as_ref().and_then(|c| c.mode.clone()).unwrap_or_else(|| "Rule".into()),
                Style::default().fg(Theme::MODE_BADGE).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let card_block = Paragraph::new(card_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(Span::styled(" 状态 Status ", Style::default().fg(Theme::TEXT_MUTED))),
        );
    f.render_widget(card_block, chunks[2]);
}
