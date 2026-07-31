use crate::app::AppState;
use crate::ui::theme::{format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),      // Current speed readout
            Constraint::Percentage(48), // Upload graph
            Constraint::Percentage(48), // Download graph
        ])
        .split(area);

    // Speed readout banner
    let readout = vec![Line::from(vec![
        Span::styled(" ▲ Upload Rate   : ", Style::default().fg(Theme::TRAFFIC_UP).add_modifier(Modifier::BOLD)),
        Span::styled(format_speed(state.current_traffic.up), Style::default().fg(Color::White)),
        Span::raw("        "),
        Span::styled(" ▼ Download Rate : ", Style::default().fg(Theme::TRAFFIC_DOWN).add_modifier(Modifier::BOLD)),
        Span::styled(format_speed(state.current_traffic.down), Style::default().fg(Color::White)),
    ])];
    let readout_block = Paragraph::new(readout).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(" Realtime Throughput Readout "),
    );
    f.render_widget(readout_block, chunks[0]);

    let up_data: Vec<u64> = state.up_history.iter().copied().collect();
    let down_data: Vec<u64> = state.down_history.iter().copied().collect();

    let up_sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" Upload Speed History (▲) "),
        )
        .data(&up_data)
        .style(Style::default().fg(Theme::TRAFFIC_UP));
    f.render_widget(up_sparkline, chunks[1]);

    let down_sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" Download Speed History (▼) "),
        )
        .data(&down_data)
        .style(Style::default().fg(Theme::TRAFFIC_DOWN));
    f.render_widget(down_sparkline, chunks[2]);
}
