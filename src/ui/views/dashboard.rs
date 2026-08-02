use crate::app::AppState;
use crate::ui::theme::{format_bytes, format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Sparkline, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Top Section: Status Diagnostics (Left) & Running Metrics (Right)
            Constraint::Length(7), // Middle Section: Realtime Traffic Sparklines
            Constraint::Length(4), // Bottom Section: System Resource Gauges (CPU, RAM, Conns)
            Constraint::Min(0),    // Footer Navigation Guide
        ])
        .split(area);

    // 1. Top Section: 60% Left Status Diagnostics, 40% Right Running Metrics
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(main_chunks[0]);

    // 1A. Left Status & Diagnostics Card
    let version_str = state
        .version
        .as_ref()
        .map(|v| format!("v{} (Meta: {})", v.version, v.meta))
        .unwrap_or_else(|| "Connecting...".into());

    let mode_str = state
        .config
        .as_ref()
        .and_then(|c| c.mode.clone())
        .unwrap_or_else(|| "Unknown".into());

    let active_node = state
        .proxies_resp
        .as_ref()
        .and_then(|r| r.proxies.get("GLOBAL"))
        .and_then(|g| g.now.clone())
        .unwrap_or_else(|| "Direct / Unknown".into());

    let sys_proxy_str = if state.is_sysproxy_enabled {
        Span::styled("● 已开启 ON", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("○ 已关闭 OFF", Style::default().fg(Theme::TEXT_MUTED))
    };

    let tun_str = if state.is_tun_enabled {
        Span::styled("● 已开启 ON", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("○ 已关闭 OFF", Style::default().fg(Theme::TEXT_MUTED))
    };

    let cap_str = if state.is_tun_privileged {
        Span::styled(" (cap_net_admin: OK)", Style::default().fg(Theme::TEXT_MUTED))
    } else {
        Span::styled(" (需 root/cap_net_admin 权限)", Style::default().fg(Color::Rgb(243, 139, 168)))
    };

    let ip_str = state
        .outbound_ip
        .as_deref()
        .unwrap_or("检测中 (Checking...)");

    let status_text = vec![
        Line::from(vec![
            Span::styled(" 核心版本 Core : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(version_str, Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 出口公网 IP   : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(ip_str, Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 运行模式 Mode : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(mode_str, Style::default().fg(Theme::MODE_BADGE).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("系统代理: ", Style::default().fg(Theme::TEXT_MUTED)),
            sys_proxy_str,
        ]),
        Line::from(vec![
            Span::styled(" TUN 虚拟网卡  : ", Style::default().fg(Theme::TEXT_MUTED)),
            tun_str,
            cap_str,
        ]),
        Line::from(vec![
            Span::styled(" GLOBAL 节点  : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(active_node, Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let status_block = Paragraph::new(status_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(" 🌐 核心与网络出口诊断 Status & Outbound "),
    );
    f.render_widget(status_block, top_chunks[0]);

    // 1B. Right Running Metrics Card
    let active_conns_count = state.connections_resp.as_ref().map(|c| c.connections.len()).unwrap_or(0);
    let rules_count = state.rules_resp.as_ref().map(|r| r.rules.len()).unwrap_or(0);
    let total_proxies = state.proxies_resp.as_ref().map(|p| p.proxies.len()).unwrap_or(0);
    let profiles_count = state.profiles.len();
    let logs_count = state.logs.len();

    let metrics_text = vec![
        Line::from(vec![
            Span::styled(" 活跃连接数 Conns  : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} 条", active_conns_count), Style::default().fg(Color::Rgb(250, 179, 135)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 订阅配置数 Sub    : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} 个", profiles_count), Style::default().fg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 代理组/节点 Proxies: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} 个", total_proxies), Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 路由规则数 Rules  : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} 条", rules_count), Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 缓存日志数 Logs   : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} 行", logs_count), Style::default().fg(Theme::TEXT_MUTED)),
        ]),
    ];

    let metrics_block = Paragraph::new(metrics_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(" 📊 运行概览 Running Metrics "),
    );
    f.render_widget(metrics_block, top_chunks[1]);

    // 2. Middle Section: Realtime Traffic Sparklines
    let graph_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    let up_data: Vec<u64> = state.up_history.iter().copied().collect();
    let down_data: Vec<u64> = state.down_history.iter().copied().collect();

    let up_max = up_data.iter().max().copied().unwrap_or(0);
    let down_max = down_data.iter().max().copied().unwrap_or(0);

    let up_title = format!(" ▲ 上行 Rate: {} | Peak: {} ", format_speed(state.current_traffic.up), format_speed(up_max));
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
    f.render_widget(up_sparkline, graph_chunks[0]);

    let down_title = format!(" ▼ 下行 Rate: {} | Peak: {} ", format_speed(state.current_traffic.down), format_speed(down_max));
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
    f.render_widget(down_sparkline, graph_chunks[1]);

    // 3. Bottom Section: Hardware System Metrics Gauges (CPU, RAM, Connections)
    let hw_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(main_chunks[2]);

    // CPU Gauge
    let cpu_ratio = (state.cpu_usage as f64 / 100.0).clamp(0.0, 1.0);
    let cpu_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(format!(" CPU 负载 ({:.1}%) ", state.cpu_usage)),
        )
        .gauge_style(Style::default().fg(Color::Rgb(203, 166, 247)).bg(Color::Rgb(30, 30, 46)))
        .ratio(cpu_ratio);
    f.render_widget(cpu_gauge, hw_chunks[0]);

    // RAM Gauge
    let ram_ratio = if state.memory_total_bytes > 0 {
        (state.memory_used_bytes as f64 / state.memory_total_bytes as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let ram_text = format!(" RAM 内存 ({}) ", format_bytes(state.memory_used_bytes));
    let ram_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(ram_text),
        )
        .gauge_style(Style::default().fg(Color::Rgb(137, 220, 235)).bg(Color::Rgb(30, 30, 46)))
        .ratio(ram_ratio);
    f.render_widget(ram_gauge, hw_chunks[1]);

    // Connection Load Gauge
    let conn_ratio = (active_conns_count as f64 / 200.0).min(1.0);
    let conn_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(format!(" 连接负载 ({}) ", active_conns_count)),
        )
        .gauge_style(Style::default().fg(Color::Rgb(250, 179, 135)).bg(Color::Rgb(30, 30, 46)))
        .ratio(conn_ratio);
    f.render_widget(conn_gauge, hw_chunks[2]);

    // 4. Footer Quick Guide
    let quick_info = vec![
        Line::from(vec![
            Span::styled(" 快捷按键 Keybindings: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(" [m] 模式切换  [p] 系统代理  [x] TUN模式  [r] 重启核心  [?] 帮助  [1-0] 页面直达 ", Style::default().fg(Color::Rgb(205, 214, 244))),
        ]),
    ];
    let info_block = Paragraph::new(quick_info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" 💡 快速提示 Guide "),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(info_block, main_chunks[3]);
}
