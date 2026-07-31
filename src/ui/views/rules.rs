use crate::app::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let header_cells = ["INDEX", "TYPE", "PAYLOAD / MATCH RULE", "PROXY TARGET"]
        .iter()
        .map(|h| ratatui::widgets::Cell::from(*h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let mut rows = Vec::new();

    if let Some(resp) = &state.rules_resp {
        for (idx, rule) in resp.rules.iter().enumerate() {
            let target_style = match rule.proxy.as_str() {
                "DIRECT" | "Direct" => Style::default().fg(Color::Green),
                "REJECT" | "Reject" => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            };

            rows.push(Row::new(vec![
                format!("#{}", idx + 1),
                rule.rule_type.clone(),
                rule.payload.clone(),
                rule.proxy.clone(),
            ]).style(Style::default()).cells(vec![
                ratatui::widgets::Cell::from(format!("#{}", idx + 1)).style(Style::default().fg(Color::DarkGray)),
                ratatui::widgets::Cell::from(rule.rule_type.clone()).style(Style::default().fg(Color::Cyan)),
                ratatui::widgets::Cell::from(rule.payload.clone()),
                ratatui::widgets::Cell::from(rule.proxy.clone()).style(target_style),
            ]));
        }
    }

    let title_str = format!(
        " Rules List Total: {} ",
        state.rules_resp.as_ref().map(|r| r.rules.len()).unwrap_or(0)
    );

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_str),
    )
    .row_highlight_style(Theme::ITEM_SELECTED);

    let mut table_state = TableState::default();
    if let Some(resp) = &state.rules_resp {
        if !resp.rules.is_empty() {
            table_state.select(Some(state.selected_rule_idx));
        }
    }

    f.render_stateful_widget(table, area, &mut table_state);
}
