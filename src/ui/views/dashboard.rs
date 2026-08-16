use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::{format_bytes, format_speed, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // 1. Core Diagnostics (Left) & System Info (Right)
            Constraint::Length(5), // 2. Clean Site Connectivity Matrix
            Constraint::Min(6),    // 3. Realtime Traffic Sparklines
        ])
        .split(area);

    // -------------------------------------------------------------------------
    // 1. TOP SECTION: Core Diagnostics (Left) & System Environment (Right)
    // -------------------------------------------------------------------------
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_chunks[0]);

    // 1A. Core Diagnostics Card
    let version_str = state
        .version
        .as_ref()
        .map(|v| format!("v{} ({})", v.version, v.meta))
        .unwrap_or_else(|| "Offline".into());

    let active_node = state
        .proxies_resp
        .as_ref()
        .and_then(|r| r.proxies.get("GLOBAL"))
        .and_then(|g| g.now.clone())
        .unwrap_or_else(|| "DIRECT".into());

    let checking_str = match lang {
        Language::Zh => "检测中...",
        Language::En => "Checking...",
    };

    let (label_ver, label_ip, label_node, label_mode, title_status) = match lang {
        Language::Zh => ("内核版本: ", "出口公网: ", "当前节点: ", "运行状态: ", " 🌐 核心与网络出口 "),
        Language::En => ("Core Ver : ", "Outbound : ", "Active   : ", "Status   : ", " 🌐 Core & Outbound "),
    };

    let tun_cap_str = if state.is_tun_enabled {
        if state.is_tun_privileged { "TUN: ON (OK)" } else { "TUN: ON (No-Priv)" }
    } else {
        "TUN: OFF"
    };

    let mode_str = state
        .config
        .as_ref()
        .and_then(|c| c.mode.clone())
        .unwrap_or_else(|| "Rule".into());

    let status_summary = format!("Mode: {} | {}", mode_str, tun_cap_str);

    let status_text = vec![
        Line::from(vec![
            Span::styled(format!(" {}", label_ver), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(version_str, Style::default().fg(Theme::ACTIVE_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_ip), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(
                state.outbound_ip.as_deref().unwrap_or(checking_str),
                Style::default().fg(Theme::WARN_YELLOW).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_node), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(active_node, Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_mode), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(status_summary, Style::default().fg(Theme::TEXT_SUB)),
        ]),
    ];

    let status_block = Paragraph::new(status_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_status),
    );
    f.render_widget(status_block, top_chunks[0]);

    // 1B. System Environment Card
    let ram_used = format_bytes(state.memory_used_bytes);
    let ram_total = format_bytes(state.memory_total_bytes);
    let ram_percent = if state.memory_total_bytes > 0 {
        (state.memory_used_bytes as f64 / state.memory_total_bytes as f64 * 100.0) as u8
    } else {
        0
    };

    let (label_host, label_cpu, label_ram, label_device, title_sysinfo, cores_str) = match lang {
        Language::Zh => ("主机环境: ", "处理器  : ", "内存占用: ", "网卡状态: ", " 💻 系统硬件与环境 ", "核"),
        Language::En => ("Host OS : ", "CPU     : ", "Memory  : ", "Device  : ", " 💻 System Info ", "cores"),
    };

    let host_kernel = format!("{} ({})", state.sys_hostname, state.sys_kernel);
    let cpu_info = format!("{} ({} {})", state.sys_cpu_brand.trim(), state.sys_cpu_cores, cores_str);
    let ram_info = format!("{} / {} ({}%)", ram_used, ram_total, ram_percent);
    let iface_info = format!("{} ({})", state.tun_interface_name, if state.is_tun_interface_up { "UP" } else { "DOWN" });

    let sys_info_text = vec![
        Line::from(vec![
            Span::styled(format!(" {}", label_host), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(host_kernel, Style::default().fg(Theme::PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_cpu), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(cpu_info, Style::default().fg(Theme::TEXT_MAIN)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_ram), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(ram_info, Style::default().fg(Theme::ACTIVE_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_device), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(iface_info, Style::default().fg(Theme::TEXT_SUB)),
        ]),
    ];

    let sys_info_block = Paragraph::new(sys_info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_sysinfo),
    );
    f.render_widget(sys_info_block, top_chunks[1]);

    // -------------------------------------------------------------------------
    // 2. MIDDLE SECTION: Clean Site Connectivity Testing Matrix (No Nested Boxes)
    // -------------------------------------------------------------------------
    let title_site = match lang {
        Language::Zh => " ⚡ 常用网站连通性 [按 't' 刷新测速] ",
        Language::En => " ⚡ Site Latency [Press 't' to Test] ",
    };

    let sites = [
        ("Google", "google.com", Color::Rgb(137, 180, 250)),
        ("GitHub", "github.com", Color::Rgb(205, 214, 244)),
        ("YouTube", "youtube.com", Color::Rgb(243, 139, 168)),
        ("OpenAI", "chatgpt.com", Color::Rgb(148, 226, 213)),
        ("Bilibili", "bilibili.com", Color::Rgb(245, 194, 231)),
        ("Baidu", "baidu.com", Color::Rgb(166, 227, 161)),
    ];

    let mut row1_spans = Vec::new();
    let mut row2_spans = Vec::new();

    row1_spans.push(Span::raw(" "));
    row2_spans.push(Span::raw(" "));

    for (idx, (site_name, _domain, brand_color)) in sites.iter().enumerate() {
        let lat_opt = state.site_latencies.get(*site_name).copied().flatten();
        let (dot_color, lat_text) = match lat_opt {
            Some(ms) if ms < 150 => (Theme::ACTIVE_GREEN, format!(" {:>3}ms ", ms)),
            Some(ms) if ms < 350 => (Theme::SECONDARY, format!(" {:>3}ms ", ms)),
            Some(ms) => (Theme::MODE_BADGE, format!(" {:>3}ms ", ms)),
            None => (Theme::DANGER_RED, "  ---ms ".to_string()),
        };

        let target_row = if idx < 3 { &mut row1_spans } else { &mut row2_spans };
        target_row.push(Span::styled("● ", Style::default().fg(dot_color)));
        target_row.push(Span::styled(format!("{:<8}", site_name), Style::default().fg(*brand_color).add_modifier(Modifier::BOLD)));
        target_row.push(Span::styled(lat_text, Style::default().fg(dot_color).add_modifier(Modifier::BOLD)));
        target_row.push(Span::raw("    "));
    }

    let site_matrix_widget = Paragraph::new(vec![
        Line::from(vec![]), // Top padding
        Line::from(row1_spans),
        Line::from(row2_spans),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_site),
    );
    f.render_widget(site_matrix_widget, main_chunks[1]);

    // -------------------------------------------------------------------------
    // 3. BOTTOM SECTION: Realtime Traffic Sparklines
    // -------------------------------------------------------------------------
    let graph_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);

    let up_data: Vec<u64> = state.up_history.iter().copied().collect();
    let down_data: Vec<u64> = state.down_history.iter().copied().collect();

    let up_max = up_data.iter().max().copied().unwrap_or(0);
    let down_max = down_data.iter().max().copied().unwrap_or(0);

    let (label_up, label_down) = match lang {
        Language::Zh => ("▲ 上行速率", "▼ 下行速率"),
        Language::En => ("▲ Upload", "▼ Download"),
    };

    let up_title = format!(" {}: {} | Peak: {} ", label_up, format_speed(state.current_traffic.up), format_speed(up_max));
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

    let down_title = format!(" {}: {} | Peak: {} ", label_down, format_speed(state.current_traffic.down), format_speed(down_max));
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
}

