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
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let groups_border_style = if state.focus_zone == crate::app::state::FocusZone::Workspace && state.proxy_sub_focus == crate::app::state::ProxySubFocus::Groups {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let nodes_border_style = if state.focus_zone == crate::app::state::FocusZone::Workspace && state.proxy_sub_focus == crate::app::state::ProxySubFocus::Nodes {
        Style::default().fg(Theme::BORDER_FOCUS)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    // 1. Render Left Pane: Proxy Groups
    let group_items: Vec<ListItem> = state
        .proxy_groups
        .iter()
        .map(|name| {
            let mut line_spans = vec![Span::raw(format!(" {} ", name))];

            if let Some(resp) = &state.proxies_resp {
                if let Some(item) = resp.proxies.get(name) {
                    if let Some(now) = &item.now {
                        line_spans.push(Span::styled(
                            format!(" → {}", now),
                            Style::default().fg(Theme::TEXT_MUTED),
                        ));
                    }
                }
            }

            ListItem::new(Line::from(line_spans))
        })
        .collect();

    let groups_list = List::new(group_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(groups_border_style)
                .title(" 代理分组 Groups "),
        )
        .highlight_style(Theme::SIDEBAR_SELECTED);

    let mut group_list_state = ListState::default();
    if !state.proxy_groups.is_empty() {
        group_list_state.select(Some(state.selected_group_idx));
    }
    f.render_stateful_widget(groups_list, chunks[0], &mut group_list_state);

    // 2. Render Right Pane: Node List with Protocol & Latency Pills
    let group_name = state.selected_group_name().unwrap_or("None");
    let current_nodes = state.current_group_nodes();
    let current_selected_now = state
        .proxies_resp
        .as_ref()
        .and_then(|r| r.proxies.get(group_name))
        .and_then(|g| g.now.as_deref());

    // Filter nodes by search query if non-empty
    let mut filtered_nodes: Vec<&String> = current_nodes
        .iter()
        .filter(|n| state.search_query.is_empty() || n.to_lowercase().contains(&state.search_query.to_lowercase()))
        .collect();

    // Sort nodes by latency if enabled
    if state.sort_nodes_by_latency {
        filtered_nodes.sort_by_key(|n| {
            let delay = state
                .latency_map
                .get(*n)
                .copied()
                .flatten()
                .or_else(|| {
                    state
                        .proxies_resp
                        .as_ref()
                        .and_then(|r| r.proxies.get(*n))
                        .and_then(|p| p.last_delay())
                });
            delay.unwrap_or(u16::MAX)
        });
    }

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            if state.is_searching { Constraint::Length(3) } else { Constraint::Length(0) },
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    // Render Search Bar if active
    if state.is_searching {
        let search_text = format!(" 🔍 Search: {}_", state.search_query);
        let search_block = Paragraph::new(search_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" 节点搜索 Search Nodes "),
            );
        f.render_widget(search_block, right_chunks[0]);
    }

    let node_items: Vec<ListItem> = filtered_nodes
        .iter()
        .map(|node_name| {
            let is_now = current_selected_now == Some(node_name.as_str());
            let mark = if is_now { "● " } else { "  " };

            // Protocol type badge lookup
            let proxy_type = state
                .proxies_resp
                .as_ref()
                .and_then(|r| r.proxies.get(*node_name))
                .map(|p| p.proxy_type.as_str())
                .unwrap_or("Node");

            let type_badge_style = match proxy_type {
                "Shadowsocks" | "SS" => Style::default().fg(Color::Rgb(137, 220, 235)),
                "Vmess" | "Vless" => Style::default().fg(Color::Rgb(203, 166, 247)),
                "Trojan" => Style::default().fg(Color::Rgb(249, 226, 175)),
                "Hysteria2" | "Tuic" => Style::default().fg(Color::Rgb(166, 227, 161)),
                "Selector" | "URLTest" => Style::default().fg(Theme::TEXT_MUTED),
                _ => Style::default().fg(Theme::TEXT_MUTED),
            };

            // Latency Pill lookup
            let delay_opt = state
                .latency_map
                .get(*node_name)
                .copied()
                .flatten()
                .or_else(|| {
                    state
                        .proxies_resp
                        .as_ref()
                        .and_then(|r| r.proxies.get(*node_name))
                        .and_then(|p| p.last_delay())
                });

            let latency_span = match delay_opt {
                Some(ms) if ms < 200 => Span::styled(
                    format!(" {:>4} ms ", ms),
                    Theme::PILL_GOOD,
                ),
                Some(ms) if ms < 500 => Span::styled(
                    format!(" {:>4} ms ", ms),
                    Theme::PILL_MEDIUM,
                ),
                Some(ms) => Span::styled(
                    format!(" {:>4} ms ", ms),
                    Theme::PILL_BAD,
                ),
                None => Span::styled(" --- ms ", Theme::PILL_UNTESTED),
            };

            let name_style = if is_now {
                Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let line = Line::from(vec![
                Span::styled(mark, Style::default().fg(Theme::ACTIVE_GREEN)),
                Span::styled(format!("[{:<7}] ", proxy_type), type_badge_style),
                Span::styled(format!("{:<30}", node_name), name_style),
                latency_span,
            ]);

            ListItem::new(line)
        })
        .collect();

    let sort_status = if state.sort_nodes_by_latency { "延迟排序:ON" } else { "默认顺序" };
    let title_str = format!(
        " 节点 Nodes in '{}' [{} 节点|{}] [Enter:选择 | d:测速 | t:全测速 | o:排序 | /:搜索] ",
        group_name,
        filtered_nodes.len(),
        sort_status
    );

    let nodes_list = List::new(node_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(nodes_border_style)
                .title(title_str),
        )
        .highlight_style(Theme::ITEM_SELECTED);

    let mut node_list_state = ListState::default();
    if !filtered_nodes.is_empty() {
        node_list_state.select(Some(state.selected_node_idx));
    }
    f.render_stateful_widget(nodes_list, right_chunks[1], &mut node_list_state);
}
