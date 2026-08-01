use crate::app::AppState;
use crate::ui::theme::{format_bytes, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Connections table
            Constraint::Length(5), // Detail Inspector Card
        ])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            if state.is_searching { Constraint::Length(3) } else { Constraint::Length(0) },
            Constraint::Min(0),
        ])
        .split(chunks[0]);

    if state.is_searching {
        let search_text = format!(" 🔍 Search Connections: {}_", state.search_query);
        let search_block = Paragraph::new(search_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" 连接搜索 Search Connections "),
            );
        f.render_widget(search_block, top_chunks[0]);
    }

    let header_cells = ["HOST / DST", "PROCESS", "RULE", "CHAINS", "UP / DOWN"]
        .iter()
        .map(|h| ratatui::widgets::Cell::from(*h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let mut conn_list: Vec<&crate::models::ConnectionItem> = Vec::new();
    if let Some(resp) = &state.connections_resp {
        for conn in &resp.connections {
            let host = conn
                .metadata
                .host
                .as_deref()
                .filter(|h| !h.is_empty())
                .or(conn.metadata.destination_ip.as_deref())
                .unwrap_or("Unknown");
            let process = conn.metadata.process.as_deref().unwrap_or("-");

            let query = state.search_query.to_lowercase();
            if query.is_empty()
                || host.to_lowercase().contains(&query)
                || process.to_lowercase().contains(&query)
                || conn.metadata.destination_ip.as_deref().unwrap_or("").contains(&query)
            {
                conn_list.push(conn);
            }
        }
    }

    if state.sort_connections_by_traffic {
        conn_list.sort_by_key(|c| std::cmp::Reverse(c.download + c.upload));
    }

    let mut rows = Vec::new();
    for conn in &conn_list {
        let host = conn
            .metadata
            .host
            .as_deref()
            .filter(|h| !h.is_empty())
            .or(conn.metadata.destination_ip.as_deref())
            .unwrap_or("Unknown");

        let process = conn
            .metadata
            .process
            .as_deref()
            .unwrap_or("-");

        let rule = conn.rule.as_deref().unwrap_or("Match");

        let chains_str = conn.chains.join(" -> ");

        let bandwidth = format!(
            "↑ {}  ↓ {}",
            format_bytes(conn.upload),
            format_bytes(conn.download)
        );

        rows.push(Row::new(vec![
            host.to_string(),
            process.to_string(),
            rule.to_string(),
            chains_str,
            bandwidth,
        ]));
    }

    let sort_label = if state.sort_connections_by_traffic { "流量降序" } else { "默认顺序" };
    let title_str = format!(
        " 活动连接 Connections ({}) [{}] ['d':断开选定 | 'D':全切断 | 's':排序 | '/':搜索] ",
        conn_list.len(),
        sort_label
    );

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_str),
    )
    .row_highlight_style(Theme::ITEM_SELECTED);

    let mut table_state = TableState::default();
    if !conn_list.is_empty() {
        table_state.select(Some(state.selected_conn_idx.min(conn_list.len() - 1)));
    }

    f.render_stateful_widget(table, top_chunks[1], &mut table_state);

    // Detail Inspector Box
    let mut detail_lines = Vec::new();
    if let Some(resp) = &state.connections_resp {
        if let Some(conn) = resp.connections.get(state.selected_conn_idx) {
            let meta = &conn.metadata;
            let host_ip = format!(
                "Host: {} ({}:{})",
                meta.host.as_deref().unwrap_or("-"),
                meta.destination_ip.as_deref().unwrap_or("-"),
                meta.destination_port.as_deref().unwrap_or("-")
            );
            let process_info = format!(
                "Process: {} ({})",
                meta.process.as_deref().unwrap_or("-"),
                meta.process_path.as_deref().unwrap_or("-")
            );
            let rule_info = format!(
                "Rule: {} Payload: [{}]",
                conn.rule.as_deref().unwrap_or("-"),
                conn.rule_payload.as_deref().unwrap_or("")
            );

            detail_lines.push(Line::from(vec![
                Span::styled(" [连接详情 Detail] ", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)),
                Span::raw(host_ip),
            ]));
            detail_lines.push(Line::from(vec![
                Span::styled(" [进程与路径 Process] ", Style::default().fg(Color::Cyan)),
                Span::raw(process_info),
            ]));
            detail_lines.push(Line::from(vec![
                Span::styled(" [匹配规则 Rule] ", Style::default().fg(Color::Yellow)),
                Span::raw(rule_info),
            ]));
        }
    }

    if detail_lines.is_empty() {
        detail_lines.push(Line::from(Span::styled("无选定连接信息 (No connection selected)", Style::default().fg(Theme::TEXT_MUTED))));
    }

    let detail_block = Paragraph::new(detail_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" Connection Detail Inspector "),
        );
    f.render_widget(detail_block, chunks[1]);
}
