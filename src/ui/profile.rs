use crate::app::AppState;
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
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let (str_add, str_update, str_del, str_storage, title_console) = match lang {
        Language::Zh => (" [ + 添加订阅 (a) ] ", " [ 🔄 更新订阅 (U) ] ", " [ 🗑️ 删除订阅 (D) ] ", "   存储: ~/.config/mimo/profiles/", " 订阅配置控制台 Subscription Profiles "),
        Language::En => (" [ + Add Profile (a) ] ", " [ 🔄 Update (U) ] ", " [ 🗑️ Delete (D) ] ", "   Storage: ~/.config/mimo/profiles/", " Subscription Profiles Console "),
    };

    // Action Header Buttons
    let btn_add = Span::styled(str_add, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD));
    let btn_update = Span::styled(str_update, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD));
    let btn_del = Span::styled(str_del, Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD));

    let info_text = vec![Line::from(vec![
        btn_add,
        Span::raw(" "),
        btn_update,
        Span::raw(" "),
        btn_del,
        Span::styled(str_storage, Style::default().fg(Theme::TEXT_MUTED)),
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
                Style::default().fg(Color::White)
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
                Span::styled(format!("{:<20}", p.name), name_style),
                Span::styled(format!(" [{:<10}]", nodes_str), Style::default().fg(Color::Rgb(137, 220, 235))),
                Span::styled(format!("  {}{:<10}", updated_label, updated_str), Style::default().fg(Theme::MODE_BADGE)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let title_str = match lang {
        Language::Zh => format!(" 订阅配置列表 ({}) [Enter:激活 | 'u':更新 | 'd':删除 | 'a':添加] ", state.profiles.len()),
        Language::En => format!(" Profiles List ({}) [Enter: Activate | 'u': Update | 'd': Delete | 'a': Add] ", state.profiles.len()),
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER_FOCUS))
                .title(title_str),
        )
        .highlight_style(Theme::SIDEBAR_SELECTED);

    let mut list_state = ListState::default();
    if !state.profiles.is_empty() {
        list_state.select(Some(state.selected_profile_idx));
    }
    f.render_stateful_widget(list, chunks[1], &mut list_state);
}
