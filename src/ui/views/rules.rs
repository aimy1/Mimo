use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

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
                ratatui::widgets::Cell::from(format!("#{}", idx + 1)).style(Style::default().fg(Color::DarkGray)),
                ratatui::widgets::Cell::from(rule.rule_type.clone()).style(Style::default().fg(Color::Cyan)),
                ratatui::widgets::Cell::from(rule.payload.clone()),
                ratatui::widgets::Cell::from(rule.proxy.clone()).style(target_style),
            ]));
        }
    } else if let Some(parsed) = &state.parsed_active_profile {
        for (idx, rule) in parsed.rules.iter().enumerate() {
            let target_style = match rule.proxy.as_str() {
                "DIRECT" | "Direct" => Style::default().fg(Color::Green),
                "REJECT" | "Reject" => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            };

            rows.push(Row::new(vec![
                ratatui::widgets::Cell::from(format!("#{}", idx + 1)).style(Style::default().fg(Color::DarkGray)),
                ratatui::widgets::Cell::from(rule.rule_type.clone()).style(Style::default().fg(Color::Cyan)),
                ratatui::widgets::Cell::from(rule.payload.clone()),
                ratatui::widgets::Cell::from(rule.proxy.clone()).style(target_style),
            ]));
        }
    }

    let rules_count = state
        .rules_resp
        .as_ref()
        .map(|r| r.rules.len())
        .or_else(|| state.parsed_active_profile.as_ref().map(|p| p.rules.len()))
        .unwrap_or(0);

    let title_str = match lang {
        Language::Zh => format!(" 路由规则列表 Rules List Total: {} [j/k: 移动滚动] ", rules_count),
        Language::En => format!(" Rules List ({}) [j/k: Scroll] ", rules_count),
    };

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

    let mut table_state = ratatui::widgets::TableState::default();
    if rules_count > 0 {
        table_state.select(Some(state.selected_rule_idx));
    }

    f.render_stateful_widget(table, area, &mut table_state);
}
