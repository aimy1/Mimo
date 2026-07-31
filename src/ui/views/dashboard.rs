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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Core Overview Status Banner
            Constraint::Length(7), // Realtime Traffic Sparklines
            Constraint::Length(4), // Hardware & System Monitors (CPU, RAM, Conns)
            Constraint::Min(0),    // Quick Navigation Guide
        ])
        .split(area);

    // 1. Core Overview Status Banner Box
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
        Span::styled("● 已开启 ENABLED", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("○ 已关闭 DISABLED", Style::default().fg(Theme::TEXT_MUTED))
    };

    let tun_str = if state.is_tun_enabled {
        Span::styled("● 已开启 ENABLED", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("○ 已关闭 DISABLED", Style::default().fg(Theme::TEXT_MUTED))
    };

    let cap_str = if state.is_tun_privileged {
        Span::styled(" (cap_net_admin: OK)", Style::default().fg(Theme::TEXT_MUTED))
    } else {
        Span::styled(" (需要 root 或 cap_net_admin 权限)", Style::default().fg(Color::Rgb(243, 139, 168)))
    };

    let status_text = vec![
        Line::from(vec![
            Span::styled(" Mihomo Core 版本 : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(version_str, Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
            Span::raw("   [按 'r' 重启 Core]"),
        ]),
        Line::from(vec![
            Span::styled(" 运行模式 Mode     : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(mode_str, Style::default().fg(Theme::MODE_BADGE).add_modifier(Modifier::BOLD)),
            Span::raw("   [按 'm' 切换 规则/全局/直连]"),
        ]),
        Line::from(vec![
            Span::styled(" 系统代理 (Linux)  : ", Style::default().fg(Theme::TEXT_MUTED)),
            sys_proxy_str,
            Span::raw("   [按 'p' 切换系统代理]"),
        ]),
        Line::from(vec![
            Span::styled(" TUN 虚拟网卡      : ", Style::default().fg(Theme::TEXT_MUTED)),
            tun_str,
            cap_str,
            Span::raw("   [按 'x' 切换 TUN 模式]"),
        ]),
        Line::from(vec![
            Span::styled(" 当前 GLOBAL 节点  : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(active_node, Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let status_block = Paragraph::new(status_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" 核心运行状态 Core Status "),
        );
    f.render_widget(status_block, chunks[0]);

    // 2. Realtime Traffic Sparklines
    let graph_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let up_data: Vec<u64> = state.up_history.iter().copied().collect();
    let down_data: Vec<u64> = state.down_history.iter().copied().collect();

    let up_max = up_data.iter().max().copied().unwrap_or(0);
    let down_max = down_data.iter().max().copied().unwrap_or(0);

    let up_title = format!(" 上行 Peak: {} | 实时 Rate: {} ", format_speed(up_max), format_speed(state.current_traffic.up));
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

    let down_title = format!(" 下行 Peak: {} | 实时 Rate: {} ", format_speed(down_max), format_speed(state.current_traffic.down));
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

    // 3. Hardware System Metrics Gauges (CPU, RAM, Connections)
    let hw_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[2]);

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
    let ram_text = format!(" 内存 RAM ({}) ", format_bytes(state.memory_used_bytes));
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
                .title(format!(" 连接负载 ({}) ", conn_count)),
        )
        .gauge_style(Style::default().fg(Color::Rgb(250, 179, 135)).bg(Color::Rgb(30, 30, 46)))
        .ratio(conn_ratio);
    f.render_widget(conn_gauge, hw_chunks[2]);

    // 4. Quick Navigation Guide
    let quick_info = vec![
        Line::from(Span::styled("快捷操控与全局热键指南:", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD))),
        Line::from("  • 按 '2' 或 鼠标点击侧边栏 -> 进入 节点选择 Panel (支持 's' 搜索, 'd' 测速)"),
        Line::from("  • 按 '3' -> 进入 订阅配置 Panel (支持 'a' 添加 URL, 'u' 更新, 'Enter' 激活)"),
        Line::from("  • 按 '4' -> 进入 规则列表 Panel (查看所有匹配 Rules)"),
        Line::from("  • 按 '5' -> 查看 活跃连接 (支持 'd' 快速关闭连接)"),
        Line::from("  • 按 '7' -> 进入 系统设置 (语言切换、端口、刷新率)"),
        Line::from("  • 按 '?' 随时调出完整帮助说明"),
    ];
    let info_block = Paragraph::new(quick_info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" 快捷按键说明 Guide "),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(info_block, chunks[3]);
}
