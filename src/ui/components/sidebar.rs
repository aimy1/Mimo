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
            Constraint::Length(4), // Logo & Author Header Banner
            Constraint::Min(0),    // Clean Iconless Navigation List
            Constraint::Length(5), // Extended Status Card
        ])
        .split(area);

    // 1. Top Logo & Author Header
    let logo_text = vec![
        Line::from(vec![
            Span::styled(" MIMO ", Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)),
            Span::styled("v0.2.0", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
        Line::from(vec![
            Span::styled(" by ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("aisaniya", Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let header_border_style = if state.focus_zone == crate::app::state::FocusZone::Sidebar {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let logo_block = Paragraph::new(logo_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(header_border_style),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(logo_block, chunks[0]);

    // 2. Iconless Clean Navigation List with Shortcut Badges
    let items: Vec<ListItem> = Tab::ALL
        .iter()
        .enumerate()
        .map(|(idx, tab)| {
            let is_selected = state.active_tab == *tab;
            let title = tab.title(lang);

            // Shortcut key index badge [1] .. [9], [0]
            let shortcut_str = if idx < 9 {
                format!("[{}] ", idx + 1)
            } else {
                "[0] ".to_string()
            };

            let (indicator, style, shortcut_style) = if is_selected {
                (
                    "▶ ",
                    Style::default()
                        .fg(Color::Rgb(17, 17, 27))
                        .bg(Color::Rgb(203, 166, 247))
                        .add_modifier(Modifier::BOLD),
                    Style::default()
                        .fg(Color::Rgb(30, 30, 46))
                        .bg(Color::Rgb(203, 166, 247))
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    "  ",
                    Style::default().fg(Color::Rgb(205, 214, 244)),
                    Style::default().fg(Color::Rgb(147, 153, 178)),
                )
            };

            let line = Line::from(vec![
                Span::styled(indicator, if is_selected { Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(203, 166, 247)) } else { Style::default().fg(Theme::TEXT_MUTED) }),
                Span::styled(shortcut_str, shortcut_style),
                Span::styled(format!("{:<10}", title.trim()), style),
            ]);

            ListItem::new(line).style(style)
        })
        .collect();

    let nav_title = match lang {
        Language::Zh => " 导航 Menu ",
        Language::En => " Navigation ",
    };

    let sidebar_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(header_border_style)
                .title(Span::styled(nav_title, Style::default().fg(Theme::TEXT_MUTED))),
        );

    let mut state_list = ListState::default();
    state_list.select(Some(state.active_tab as usize));
    f.render_stateful_widget(sidebar_list, chunks[1], &mut state_list);

    // 3. Extended Status Card
    let active_profile = state
        .profiles
        .iter()
        .find(|p| p.is_active)
        .map(|p| p.name.as_str())
        .unwrap_or("Default");

    let tun_status_span = if state.is_tun_enabled {
        Span::styled("TUN: ON", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("TUN: OFF", Style::default().fg(Theme::TEXT_MUTED))
    };

    let mode_str = state
        .config
        .as_ref()
        .and_then(|c| c.mode.clone())
        .unwrap_or_else(|| "Rule".into());

    let (label_sub, label_mode, label_outbound, title_status, checking_str) = match lang {
        Language::Zh => (" 订阅: ", " 模式: ", " 出口: ", " 状态 Status ", "检测中..."),
        Language::En => (" Sub : ", " Mode: ", " Out : ", " Status ", "Checking..."),
    };

    let card_text = vec![
        Line::from(vec![
            Span::styled(label_sub, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(active_profile, Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(label_mode, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(mode_str, Style::default().fg(Theme::MODE_BADGE).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            tun_status_span,
        ]),
        Line::from(vec![
            Span::styled(label_outbound, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(
                state.outbound_ip.as_deref().unwrap_or(checking_str),
                Style::default().fg(Color::Rgb(249, 226, 175)),
            ),
        ]),
    ];

    let card_block = Paragraph::new(card_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(Span::styled(title_status, Style::default().fg(Theme::TEXT_MUTED))),
        );
    f.render_widget(card_block, chunks[2]);
}
