use crate::app::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let popup_area = centered_rect(65, 45, area);
    f.render_widget(Clear, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Name input
            Constraint::Length(3), // URL input
            Constraint::Min(0),    // Instructions
        ])
        .split(popup_area);

    // Name Input Block
    let name_border_style = if state.profile_input_focus == 0 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let name_widget = Paragraph::new(state.profile_name_input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(name_border_style)
                .title(" Profile Name "),
        );
    f.render_widget(name_widget, chunks[0]);

    // URL Input Block
    let url_border_style = if state.profile_input_focus == 1 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let url_widget = Paragraph::new(state.profile_url_input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(url_border_style)
                .title(" Subscription URL (http/https) "),
        );
    f.render_widget(url_widget, chunks[1]);

    // Instruction Footer
    let text = vec![
        Line::from(vec![
            Span::styled(" Tab", Style::default().fg(Color::Yellow)),
            Span::raw(": Switch Input Field | "),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Download & Save | "),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(": Cancel"),
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
