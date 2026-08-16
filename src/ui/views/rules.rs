use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let header_cells = ["INDEX", "TYPE", "PAYLOAD / MATCH RULE", "PROXY TARGET"]
        .iter()
        .map(|h| ratatui::widgets::Cell::from(*h).style(Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let mut rows = Vec::new();

    let push_rule = |rows: &mut Vec<Row>, idx: usize, rule_type: &str, payload: &str, proxy: &str| {
        let target_style = match proxy {
            "DIRECT" | "Direct" => Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD),
            "REJECT" | "Reject" => Style::default().fg(Theme::DANGER_RED).add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
        };

        rows.push(Row::new(vec![
            ratatui::widgets::Cell::from(format!("#{}", idx + 1)).style(Style::default().fg(Theme::TEXT_DIM)),
            ratatui::widgets::Cell::from(rule_type.to_string()).style(Style::default().fg(Theme::SECONDARY)),
            ratatui::widgets::Cell::from(payload.to_string()).style(Style::default().fg(Theme::TEXT_MAIN)),
            ratatui::widgets::Cell::from(proxy.to_string()).style(target_style),
        ]));
    };

    if let Some(resp) = &state.rules_resp {
        for (idx, rule) in resp.rules.iter().enumerate() {
            push_rule(&mut rows, idx, &rule.rule_type, &rule.payload, &rule.proxy);
        }
    } else if let Some(parsed) = &state.parsed_active_profile {
        for (idx, rule) in parsed.rules.iter().enumerate() {
            push_rule(&mut rows, idx, &rule.rule_type, &rule.payload, &rule.proxy);
        }
    }

    let rules_count = state
        .rules_resp
        .as_ref()
        .map(|r| r.rules.len())
        .or_else(|| state.parsed_active_profile.as_ref().map(|p| p.rules.len()))
        .unwrap_or(0);

    let title_str = match lang {
        Language::Zh => format!(" 📋 路由规则 ({} 条) [j/k: 滚动] ", rules_count),
        Language::En => format!(" 📋 Rules ({}) [j/k: Scroll] ", rules_count),
    };

    let border_style = if state.focus_zone == crate::app::state::FocusZone::Workspace {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
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
            .border_style(border_style)
            .title(title_str),
    )
    .row_highlight_style(Theme::ITEM_SELECTED);

    let mut table_state = ratatui::widgets::TableState::default();
    if rules_count > 0 {
        table_state.select(Some(state.selected_rule_idx));
    }

    f.render_stateful_widget(table, area, &mut table_state);
}

