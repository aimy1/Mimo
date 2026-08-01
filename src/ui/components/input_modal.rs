use crate::app::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let popup_area = centered_rect(68, 48, area);
    f.render_widget(Clear, popup_area);

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::BORDER_FOCUS))
        .title(Span::styled(" ➕ 添加/编辑 机场订阅 Profile ", Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)));
    
    let inner_area = modal_block.inner(popup_area);
    f.render_widget(modal_block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Name input
            Constraint::Length(3), // URL input
            Constraint::Min(0),    // Instructions
        ])
        .split(inner_area);

    // Name Input Block
    let is_name_focused = state.profile_input_focus == 0;
    let name_border_style = if is_name_focused {
        Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let name_display = if state.profile_name_input.is_empty() {
        if is_name_focused {
            format!(" 请输入订阅名称 (例如: MySub)_")
        } else {
            format!(" 请输入订阅名称")
        }
    } else {
        let cursor = if is_name_focused { "_" } else { "" };
        format!(" {}{}", state.profile_name_input, cursor)
    };

    let name_widget = Paragraph::new(name_display)
        .style(if state.profile_name_input.is_empty() && is_name_focused { Style::default().fg(Color::Rgb(147, 153, 178)) } else { Style::default().fg(Color::White) })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(name_border_style)
                .title(" 1. 订阅名称 (Profile Name) "),
        );
    f.render_widget(name_widget, chunks[0]);

    // URL Input Block
    let is_url_focused = state.profile_input_focus == 1;
    let url_border_style = if is_url_focused {
        Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let url_display = if state.profile_url_input.is_empty() {
        if is_url_focused {
            format!(" 请粘贴订阅 URL (http:// 或 https://)_")
        } else {
            format!(" 请粘贴订阅 URL")
        }
    } else {
        let cursor = if is_url_focused { "_" } else { "" };
        format!(" {}{}", state.profile_url_input, cursor)
    };

    let url_widget = Paragraph::new(url_display)
        .style(if state.profile_url_input.is_empty() && is_url_focused { Style::default().fg(Color::Rgb(147, 153, 178)) } else { Style::default().fg(Color::White) })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(url_border_style)
                .title(" 2. 订阅链接 (Subscription URL / Local File Path) "),
        );
    f.render_widget(url_widget, chunks[1]);

    // Instruction Footer
    let text = vec![
        Line::from(vec![
            Span::styled(" [ Tab ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
            Span::raw(" 切换输入框   "),
            Span::styled(" [ Enter ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
            Span::raw(" 下载并解析   "),
            Span::styled(" [ Esc ] ", Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD)),
            Span::raw(" 取消 "),
        ]),
    ];
    let footer_widget = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);
    f.render_widget(footer_widget, chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
