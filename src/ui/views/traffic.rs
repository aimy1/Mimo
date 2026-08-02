use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::{format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),      // Current speed readout
            Constraint::Percentage(48), // Upload graph
            Constraint::Percentage(48), // Download graph
        ])
        .split(area);

    let (label_up, label_down, title_readout, title_up, title_down) = match lang {
        Language::Zh => (" ▲ 实时上行速率 : ", " ▼ 实时下行速率 : ", " 实时吞吐数据 Readout ", " 上行速率历史 (▲) ", " 下行速率历史 (▼) "),
        Language::En => (" ▲ Upload Rate   : ", " ▼ Download Rate : ", " Realtime Throughput Readout ", " Upload Speed History (▲) ", " Download Speed History (▼) "),
    };

    // Speed readout banner
    let readout = vec![Line::from(vec![
        Span::styled(label_up, Style::default().fg(Theme::TRAFFIC_UP).add_modifier(Modifier::BOLD)),
        Span::styled(format_speed(state.current_traffic.up), Style::default().fg(Color::White)),
        Span::raw("        "),
        Span::styled(label_down, Style::default().fg(Theme::TRAFFIC_DOWN).add_modifier(Modifier::BOLD)),
        Span::styled(format_speed(state.current_traffic.down), Style::default().fg(Color::White)),
    ])];
    let readout_block = Paragraph::new(readout).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_readout),
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
                .title(title_up),
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
                .title(title_down),
        )
        .data(&down_data)
        .style(Style::default().fg(Theme::TRAFFIC_DOWN));
    f.render_widget(down_sparkline, chunks[2]);
}
