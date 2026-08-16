use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::{format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Current speed readout
            Constraint::Percentage(50), // Upload graph
            Constraint::Percentage(50), // Download graph
        ])
        .split(area);

    let (label_up, label_down, title_readout, title_up, title_down) = match lang {
        Language::Zh => ("▲ 实时上行: ", "▼ 实时下行: ", " 实时网络速率 ", " ▲ 上行速率历史 ", " ▼ 下行速率历史 "),
        Language::En => ("▲ Upload: ", "▼ Download: ", " Realtime Throughput ", " ▲ Upload History ", " ▼ Download History "),
    };


    // Speed readout banner
    let readout = vec![Line::from(vec![
        Span::styled(format!(" {}", label_up), Style::default().fg(Theme::TRAFFIC_UP).add_modifier(Modifier::BOLD)),
        Span::styled(format!("{:<14}", format_speed(state.current_traffic.up)), Style::default().fg(Theme::TEXT_MAIN).add_modifier(Modifier::BOLD)),
        Span::raw("        "),
        Span::styled(label_down, Style::default().fg(Theme::TRAFFIC_DOWN).add_modifier(Modifier::BOLD)),
        Span::styled(format_speed(state.current_traffic.down), Style::default().fg(Theme::TEXT_MAIN).add_modifier(Modifier::BOLD)),
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

    let up_max = up_data.iter().max().copied().unwrap_or(0);
    let down_max = down_data.iter().max().copied().unwrap_or(0);

    let up_title = format!("{}| 当前: {} | 峰值: {} ", title_up, format_speed(state.current_traffic.up), format_speed(up_max));
    let up_sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(up_title),
        )
        .data(&up_data)
        .style(Style::default().fg(Theme::TRAFFIC_UP));
    f.render_widget(up_sparkline, chunks[1]);

    let down_title = format!("{}| 当前: {} | 峰值: {} ", title_down, format_speed(state.current_traffic.down), format_speed(down_max));
    let down_sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(down_title),
        )
        .data(&down_data)
        .style(Style::default().fg(Theme::TRAFFIC_DOWN));
    f.render_widget(down_sparkline, chunks[2]);
}
