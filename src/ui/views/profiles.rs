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

    let (str_add, str_update, str_del, str_dir, title_console) = match lang {
        Language::Zh => (" [ + 添加订阅 (a) ] ", " [ 🔄 更新订阅 (u) ] ", " [ 🗑️ 删除 (d) ] ", "   目录: ~/.config/mimo/profiles/", " 订阅配置控制台 Profiles Manager "),
        Language::En => (" [ + Add Profile (a) ] ", " [ 🔄 Update (u) ] ", " [ 🗑️ Delete (d) ] ", "   Dir: ~/.config/mimo/profiles/", " Subscription Profiles Manager "),
    };

    // Clickable Action Header Buttons
    let btn_add = Span::styled(str_add, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD));
    let btn_update = Span::styled(str_update, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD));
    let btn_del = Span::styled(str_del, Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD));

    let info_text = vec![Line::from(vec![
        btn_add,
        Span::raw(" "),
        btn_update,
        Span::raw(" "),
        btn_del,
        Span::styled(str_dir, Style::default().fg(Theme::TEXT_MUTED)),
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

    let (just_now_str, min_ago_str, hr_ago_str, day_ago_str, never_str, local_file_str, updated_label) = match lang {
        Language::Zh => ("刚刚", "分钟前", "小时前", "天前", "从未", "[本地配置文件]", " 更新: "),
        Language::En => ("Just now", "m ago", "h ago", "d ago", "Never", "[Local File]", " Updated: "),
    };

    // Profiles List
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

            let url_display = p.url.as_deref().unwrap_or(local_file_str);

            let line = Line::from(vec![
                Span::styled(mark, mark_style),
                Span::styled(format!("{:<20}", p.name), name_style),
                Span::styled(format!(" {:<45}", url_display), Style::default().fg(Theme::TEXT_MUTED)),
                Span::styled(format!(" {}{:<10}", updated_label, updated_str), Style::default().fg(Theme::MODE_BADGE)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let title_str = match lang {
        Language::Zh => format!(" 本地订阅列表 ({}) [点击/Enter:激活 | 'a':添加 | 'u':更新 | 'd':删除] ", state.profiles.len()),
        Language::En => format!(" Profiles List ({}) [Click/Enter: Activate | 'a': Add | 'u': Update | 'd': Delete] ", state.profiles.len()),
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
