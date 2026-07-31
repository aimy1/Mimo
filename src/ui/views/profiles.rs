use crate::app::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Clickable Action Header Buttons
    let btn_add = Span::styled(" [ + 添加订阅 (a) ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD));
    let btn_update = Span::styled(" [ 🔄 更新订阅 (u) ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD));
    let btn_del = Span::styled(" [ 🗑️ 删除 (d) ] ", Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD));

    let info_text = vec![Line::from(vec![
        btn_add,
        Span::raw(" "),
        btn_update,
        Span::raw(" "),
        btn_del,
        Span::styled("   目录: ~/.config/mimo/profiles/", Style::default().fg(Theme::TEXT_MUTED)),
    ])];
    let info_block = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" 订阅配置控制台 Profiles Manager "),
        );
    f.render_widget(info_block, chunks[0]);

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
                        "刚刚".to_string()
                    } else if diff < 3600 {
                        format!("{}分钟前", diff / 60)
                    } else if diff < 86400 {
                        format!("{}小时前", diff / 3600)
                    } else {
                        format!("{}天前", diff / 86400)
                    }
                })
                .unwrap_or_else(|| "从未".to_string());

            let url_display = p.url.as_deref().unwrap_or("[本地配置文件]");

            let line = Line::from(vec![
                Span::styled(mark, mark_style),
                Span::styled(format!("{:<20}", p.name), name_style),
                Span::styled(format!(" {:<45}", url_display), Style::default().fg(Theme::TEXT_MUTED)),
                Span::styled(format!(" 更新: {:<10}", updated_str), Style::default().fg(Theme::MODE_BADGE)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let title_str = format!(" 本地订阅列表 ({}) [点击/Enter:激活 | 'a':添加 | 'u':更新 | 'd':删除] ", state.profiles.len());
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
