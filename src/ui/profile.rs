use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let (str_add, str_update, str_del, title_console) = match lang {
        Language::Zh => ("a: 添加订阅", "u: 更新订阅", "d: 删除订阅", " 📁 订阅管理 "),
        Language::En => ("a: Add", "u: Update", "d: Delete", " 📁 Profiles "),
    };

    let btn_add = Span::styled(format!(" [{}] ", str_add), Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD));
    let btn_update = Span::styled(format!(" [{}] ", str_update), Style::default().fg(Theme::WARN_YELLOW).add_modifier(Modifier::BOLD));
    let btn_del = Span::styled(format!(" [{}] ", str_del), Style::default().fg(Theme::DANGER_RED).add_modifier(Modifier::BOLD));

    let info_text = vec![Line::from(vec![
        Span::raw(" "),
        btn_add,
        Span::raw(" "),
        btn_update,
        Span::raw(" "),
        btn_del,
        Span::styled("   ~/.config/mimo/profiles/", Style::default().fg(Theme::TEXT_DIM)),
    ])];

    let info_block = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(title_console),
        );
    f.render_widget(info_block, chunks[0]);

    let (just_now_str, min_ago_str, hr_ago_str, day_ago_str, never_str, updated_label, nodes_label) = match lang {
        Language::Zh => ("刚刚", "分钟前", "小时前", "天前", "从未", "更新: ", "节点"),
        Language::En => ("Just now", "m ago", "h ago", "d ago", "Never", "Updated: ", "nodes"),
    };

    // Profiles List: Airport Name, Node Count, Update Time
    let items: Vec<ListItem> = state
        .profiles
        .iter()
        .map(|p| {
            let mark = if p.is_active { "● " } else { "  " };
            let mark_style = if p.is_active {
                Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::TEXT_MUTED)
            };

            let name_style = if p.is_active {
                Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::TEXT_MAIN)
            };

            let updated_str = p
                .updated_at
                .map(|t| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    let diff = now.saturating_sub(t);
                    if diff < 60 {
                        just_now_str.to_string()
                    } else if diff < 3600 {
                        format!("{}{}", diff / 60, min_ago_str)
                    } else if diff < 86400 {
                        format!("{}{}", diff / 3600, hr_ago_str)
                    } else {
                        format!("{}{}", diff / 86400, day_ago_str)
                    }
                })
                .unwrap_or_else(|| never_str.to_string());

            let nodes_str = format!("{} {}", p.node_count, nodes_label);

            let line = Line::from(vec![
                Span::styled(mark, mark_style),
                Span::styled(format!("{:<24}", p.name), name_style),
                Span::styled(format!(" [{:<8}]", nodes_str), Style::default().fg(Theme::SECONDARY)),
                Span::styled(format!("   {}{:<12}", updated_label, updated_str), Style::default().fg(Theme::TEXT_MUTED)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let title_str = match lang {
        Language::Zh => format!(" 订阅列表 ({}) [Enter:激活 | u:更新 | d:删除 | a:添加] ", state.profiles.len()),
        Language::En => format!(" Profiles ({}) [Enter:Activate | u:Update | d:Delete | a:Add] ", state.profiles.len()),
    };

    let border_style = if state.focus_zone == crate::app::state::FocusZone::Workspace {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title(title_str),
        )
        .highlight_style(Theme::SIDEBAR_SELECTED);

    let mut list_state = ListState::default();
    if !state.profiles.is_empty() {
        list_state.select(Some(state.selected_profile_idx));
    }
    f.render_stateful_widget(list, chunks[1], &mut list_state);
}

