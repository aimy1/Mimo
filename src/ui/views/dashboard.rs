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
            Constraint::Length(8), // Top: Diagnostics & Core Status Grid (2 Cards)
            Constraint::Length(7), // Middle: Realtime Traffic Sparklines (2 Cards)
            Constraint::Length(4), // Hardware & Load Gauges (3 Cards)
            Constraint::Min(0),    // Bottom: Health & Quick Navigation
        ])
        .split(area);

    // ==========================================
    // 1. TOP SECTION: 2-COLUMN GRID DIAGNOSTICS
    // ==========================================
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    // --- Card 1: Core & Network Diagnostics ---
    let version_str = state
        .version
        .as_ref()
        .map(|v| format!("v{} (Meta: {})", v.version, v.meta))
        .unwrap_or_else(|| "离线 Offline".into());

    let mode_str = state
        .config
        .as_ref()
        .and_then(|c| c.mode.clone())
        .unwrap_or_else(|| "Rule".into());

    let active_node = state
        .proxies_resp
        .as_ref()
        .and_then(|r| r.proxies.get("GLOBAL"))
        .and_then(|g| g.now.clone())
        .unwrap_or_else(|| "Direct / 直连".into());

    let ip_str = state
        .outbound_ip
        .as_deref()
        .unwrap_or("检测中 Checking...");

    let card1_text = vec![
        Line::from(vec![
            Span::styled(" 🚀 Mihomo 核心 : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(version_str, Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 🌐 出口公网 IP : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(ip_str, Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" ⚡ 运行模式 Mode: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!(" {} ", mode_str), Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::MODE_BADGE).add_modifier(Modifier::BOLD)),
            Span::styled(" (按 'm' 切换)", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
        Line::from(vec![
            Span::styled(" 🎯 GLOBAL 节点  : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(active_node, Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let card1_block = Paragraph::new(card1_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER_FOCUS))
            .title(" 🚀 核心与出口状态 Diagnostics "),
    );
    f.render_widget(card1_block, top_chunks[0]);

    // --- Card 2: Proxy & TUN Services ---
    let sys_proxy_span = if state.is_sysproxy_enabled {
        Span::styled("● 已开启 ENABLED", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("○ 已关闭 DISABLED", Style::default().fg(Theme::TEXT_MUTED))
    };

    let tun_span = if state.is_tun_enabled {
        Span::styled("● 已开启 ENABLED", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("○ 已关闭 DISABLED", Style::default().fg(Theme::TEXT_MUTED))
    };

    let cap_span = if state.is_tun_privileged {
        Span::styled(" (cap_net_admin: OK)", Style::default().fg(Color::Rgb(166, 227, 161)))
    } else {
        Span::styled(" (需要 root 提权)", Style::default().fg(Color::Rgb(243, 139, 168)))
    };

    let ports_str = format!("HTTP: {} | SOCKS: {}", state.settings_http_port, state.settings_socks_port);

    let card2_text = vec![
        Line::from(vec![
            Span::styled(" 🖥️ 桌面系统代理: ", Style::default().fg(Theme::TEXT_MUTED)),
            sys_proxy_span,
            Span::styled(" (按 'p' 开关)", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
        Line::from(vec![
            Span::styled(" 🛡️ TUN 虚拟网卡 : ", Style::default().fg(Theme::TEXT_MUTED)),
            tun_span,
            cap_span,
        ]),
        Line::from(vec![
            Span::styled(" ⚙️ TUN 协议栈   : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(state.settings_tun_stack.as_str(), Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 🔌 本地代理端口 : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(ports_str, Style::default().fg(Color::Rgb(250, 179, 135))),
        ]),
    ];

    let card2_block = Paragraph::new(card2_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(" 🛡️ 代理与系统接管 Services "),
    );
    f.render_widget(card2_block, top_chunks[1]);

    // ==========================================
    // 2. MIDDLE SECTION: DUAL REALTIME SPARKLINES
    // ==========================================
    let graph_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    let up_data: Vec<u64> = state.up_history.iter().copied().collect();
    let down_data: Vec<u64> = state.down_history.iter().copied().collect();

    let up_max = up_data.iter().max().copied().unwrap_or(0);
    let down_max = down_data.iter().max().copied().unwrap_or(0);

    let up_title = format!(" ⬆️ 上行速率 Rate: {} | 峰值 Peak: {} ", format_speed(state.current_traffic.up), format_speed(up_max));
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

    let down_title = format!(" ⬇️ 下行速率 Rate: {} | 峰值 Peak: {} ", format_speed(state.current_traffic.down), format_speed(down_max));
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

    // ==========================================
    // 3. HARDWARE & LOAD MONITORS (3 GAUGES)
    // ==========================================
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
                .title(format!(" 💻 CPU 占用 ({:.1}%) ", state.cpu_usage)),
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
    let ram_text = format!(" 🧠 内存占用 ({}) ", format_bytes(state.memory_used_bytes));
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
    let conn_count = state.connections_resp.as_ref().map(|c| c.connections.len()).unwrap_or(0);
    let conn_ratio = (conn_count as f64 / 200.0).min(1.0);
    let conn_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(format!(" 🔌 活动连接数 ({}) ", conn_count)),
        )
        .gauge_style(Style::default().fg(Color::Rgb(250, 179, 135)).bg(Color::Rgb(30, 30, 46)))
        .ratio(conn_ratio);
    f.render_widget(conn_gauge, hw_chunks[2]);

    // ==========================================
    // 4. BOTTOM SECTION: QUICK HELP & NAVIGATION
    // ==========================================
    let quick_info = vec![
        Line::from(vec![
            Span::styled(" ⚡ 核心快捷键 Hotkeys : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(" [m] Mode  [p] SysProxy  [x] TUN  [r] Restart  [?] Help  [q] Quit ", Style::default().fg(Color::Rgb(205, 214, 244)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 📌 页面直达 Tab Jump   : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(" [1] Dashboard  [2] Proxies  [3] Profiles  [4] Rules  [5] Connections  [8] Settings ", Style::default().fg(Color::Rgb(147, 153, 178))),
        ]),
    ];
    let info_block = Paragraph::new(quick_info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" 💡 交互提示 Guide "),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(info_block, main_chunks[3]);
}
