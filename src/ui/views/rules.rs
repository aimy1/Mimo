use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table},
    Frame,
};

pub fn render(f: &mut Frame, state: &mut AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            if state.is_rules_searching { Constraint::Length(3) } else { Constraint::Length(0) },
            Constraint::Min(0),
        ])
        .split(area);

    if state.is_rules_searching {
        let search_title = match lang {
            Language::Zh => " 搜索路由规则 ",
            Language::En => " Search Routing Rules ",
        };
        let search_text = format!(" 搜索: {}█", state.rules_search_query);
        let search_block = Paragraph::new(search_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Theme::PRIMARY))
                    .title(search_title),
            );
        f.render_widget(search_block, chunks[0]);
    }

    let header_cells = ["INDEX", "TYPE", "PAYLOAD / MATCH RULE", "PROXY TARGET"]
        .iter()
        .map(|h| ratatui::widgets::Cell::from(*h).style(Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let filtered_rules = state.filtered_rules();
    let total_count = state
        .rules_resp
        .as_ref()
        .map(|r| r.rules.len())
        .or_else(|| state.parsed_active_profile.as_ref().map(|p| p.rules.len()))
        .unwrap_or(0);

    let mut rows = Vec::new();
    for (orig_idx, rule_type, payload, proxy) in &filtered_rules {
        let target_style = match proxy.as_str() {
            "DIRECT" | "Direct" => Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD),
            "REJECT" | "Reject" => Style::default().fg(Theme::DANGER_RED).add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
        };

        rows.push(Row::new(vec![
            ratatui::widgets::Cell::from(format!("#{}", orig_idx + 1)).style(Style::default().fg(Theme::TEXT_DIM)),
            ratatui::widgets::Cell::from(rule_type.clone()).style(Style::default().fg(Theme::SECONDARY)),
            ratatui::widgets::Cell::from(payload.clone()).style(Style::default().fg(Theme::TEXT_MAIN)),
            ratatui::widgets::Cell::from(proxy.clone()).style(target_style),
        ]));
    }

    let title_str = match lang {
        Language::Zh => {
            if state.rules_search_query.is_empty() {
                format!(" 路由规则 ({} 条) [/ 或 s: 搜索 | j/k: 滚动] ", total_count)
            } else {
                format!(" 路由规则 (匹配 {} / 全部 {} 条) [/ 或 s: 搜索] ", filtered_rules.len(), total_count)
            }
        }
        Language::En => {
            if state.rules_search_query.is_empty() {
                format!(" Rules ({}) [/ or s: Search | j/k: Scroll] ", total_count)
            } else {
                format!(" Rules (Matched {} / Total {}) [/ or s: Search] ", filtered_rules.len(), total_count)
            }
        }
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

    if filtered_rules.is_empty() {
        state.rules_state.select(None);
    } else {
        state.rules_state.select(Some(state.selected_rule_idx.min(filtered_rules.len() - 1)));
    }

    f.render_stateful_widget(table, chunks[1], &mut state.rules_state);
}
