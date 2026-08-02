use crate::app::AppState;
use crate::ui::theme::{format_bytes, format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Top Section: Core Diagnostics (Left) & System Info (Right)
            Constraint::Length(7), // Middle 1 Section: Beautified Site & Service Connectivity Testing
            Constraint::Length(7), // Middle 2 Section: Realtime Traffic Sparklines
            Constraint::Min(0),    // Footer Navigation Guide
        ])
        .split(area);

    // 1. Top Section: 50% Left Core Diagnostics, 50% Right System Info
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
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

    // 1B. Right System Information Card
    let ram_used = format_bytes(state.memory_used_bytes);
    let ram_total = format_bytes(state.memory_total_bytes);
    let ram_percent = if state.memory_total_bytes > 0 {
        (state.memory_used_bytes as f64 / state.memory_total_bytes as f64 * 100.0) as u8
    } else {
        0
    };

    let sys_info_text = vec![
        Line::from(vec![
            Span::styled(" 主机名称 Hostname : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(&state.sys_hostname, Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" Linux 内核 Kernel: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(&state.sys_kernel, Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 处理器 CPU 架构  : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} ({} 核心)", state.sys_cpu_brand.trim(), state.sys_cpu_cores), Style::default().fg(Color::Rgb(250, 179, 135)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 物理内存 RAM     : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} / {} ({}%)", ram_used, ram_total, ram_percent), Style::default().fg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" TUN 网卡设备     : ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("{} ({})", state.tun_interface_name, if state.is_tun_interface_up { "UP 活跃" } else { "DOWN 停用" }), Style::default().fg(Theme::TEXT_MUTED)),
        ]),
    ];

    let sys_info_block = Paragraph::new(sys_info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(" 💻 详细系统环境信息 System Info "),
    );
    f.render_widget(sys_info_block, top_chunks[1]);

    // 2. Middle 1 Section: Beautified Site & Service Connectivity Testing Cards Grid
    let site_outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(203, 166, 247)))
        .title(Span::styled(
            " 🌐 常用网站与服务连通性测试 Site & Service Connectivity [按 't' 键 / 点击刷新] ",
            Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(site_outer_block, main_chunks[1]);

    let inner_site_area = Rect {
        x: main_chunks[1].x + 1,
        y: main_chunks[1].y + 1,
        width: main_chunks[1].width.saturating_sub(2),
        height: main_chunks[1].height.saturating_sub(2),
    };

    let sites = [
        ("Google", "google.com", Color::Rgb(137, 180, 250)),
        ("GitHub", "github.com", Color::Rgb(205, 214, 244)),
        ("YouTube", "youtube.com", Color::Rgb(243, 139, 168)),
        ("OpenAI", "chatgpt.com", Color::Rgb(148, 226, 213)),
        ("Bilibili", "bilibili.com", Color::Rgb(245, 194, 231)),
        ("Baidu", "baidu.com", Color::Rgb(166, 227, 161)),
    ];

    let site_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(18),
            Constraint::Percentage(18),
        ])
        .split(inner_site_area);

    for (idx, (site_name, domain, brand_color)) in sites.iter().enumerate() {
        if let Some(target_area) = site_chunks.get(idx) {
            let lat_opt = state.site_latencies.get(*site_name).copied().flatten();
            let (status_str, style) = match lat_opt {
                Some(ms) if ms < 100 => (format!("● {} ms [极速]", ms), Style::default().fg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD)),
                Some(ms) if ms < 300 => (format!("● {} ms [良好]", ms), Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
                Some(ms) => (format!("● {} ms [一般]", ms), Style::default().fg(Color::Rgb(250, 179, 135)).add_modifier(Modifier::BOLD)),
                None => ("○ 测试中/超时".into(), Style::default().fg(Color::Rgb(243, 139, 168))),
            };

            let text = vec![
                Line::from(Span::styled(*site_name, Style::default().fg(*brand_color).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(*domain, Style::default().fg(Theme::TEXT_MUTED))),
                Line::from(Span::styled(status_str, style)),
            ];

            let card = Paragraph::new(text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Theme::BORDER)),
            );
            f.render_widget(card, *target_area);
        }
    }

    // 3. Middle 2 Section: Realtime Traffic Sparklines
    let graph_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);

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

    // 4. Footer Quick Guide
    let quick_info = vec![
        Line::from(vec![
            Span::styled(" 快捷按键 Keybindings: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(" [m] 模式切换  [p] 系统代理  [x] TUN模式  [t] 网站/节点测速  [r] 重启核心  [?] 帮助  [1-0] 页面直达 ", Style::default().fg(Color::Rgb(205, 214, 244))),
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
