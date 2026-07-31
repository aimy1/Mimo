use crate::app::AppState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),  // Header description
            Constraint::Length(12), // TUN privilege card
            Constraint::Length(8),  // GSettings card
            Constraint::Min(4),     // Action & Instructions
        ])
        .split(area);

    // 1. Header description
    let header_text = vec![
        Line::from(vec![
            Span::styled(" 🔒 系统权限管理与提权控制中心 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 本页面用于管理 Mimo 及 Mihomo 核心所需的 Linux 系统高级接口与 TUN 虚拟网卡权限。", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let header_p = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER)),
    );
    f.render_widget(header_p, chunks[0]);

    // 2. TUN Privilege Card
    let is_auth = state.is_tun_privileged;
    let auth_badge = if is_auth {
        Span::styled(" [ ✔ 已授权 AUTHORIZED ] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [ ✖ 未授权 UNAUTHORIZED ] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
    };

    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "未知路径".to_string());

    let tun_card_text = vec![
        Line::from(vec![
            Span::styled("权限名称: ", Style::default().fg(Color::Cyan)),
            Span::raw("Linux CAP_NET_ADMIN (TUN 虚拟网卡接口创建与绑定)"),
            Span::raw("   状态: "),
            auth_badge,
        ]),
        Line::from(vec![
            Span::styled("核心程序路径: ", Style::default().fg(Color::Cyan)),
            Span::styled(binary_path, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("权限说明: ", Style::default().fg(Color::Cyan)),
            Span::raw("TUN 模式需要向系统申请网络设备管理 Capability 权限，允许接管全局 IP 协议栈流量。"),
        ]),
        Line::from(vec![]),
        Line::from(vec![
            Span::styled(" [ 🔑 授权系统 Root 权限 (按 P 键 / 单击) ] ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("   "),
            Span::styled(" [ 🚫 撤销系统权限 (按 R 键) ] ", Style::default().fg(Color::White).bg(Color::Red)),
        ]),
    ];

    let tun_card = Paragraph::new(tun_card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if is_auth { Color::Green } else { Color::Red }))
            .title(" 1. TUN 虚拟网卡提权 (CAP_NET_ADMIN) "),
    );
    f.render_widget(tun_card, chunks[1]);

    // 3. System Proxy GSettings Card
    let sysproxy_card_text = vec![
        Line::from(vec![
            Span::styled("权限名称: ", Style::default().fg(Color::Cyan)),
            Span::raw("GNOME Desktop GSettings 系统代理修改权限"),
            Span::raw("   状态: "),
            Span::styled(" [ ✔ 正常 READY ] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("作用范围: ", Style::default().fg(Color::Cyan)),
            Span::raw("org.gnome.system.proxy (HTTP/HTTPS/SOCKS 端口控制)"),
        ]),
        Line::from(vec![
            Span::styled("权限说明: ", Style::default().fg(Color::Cyan)),
            Span::raw("允许按 'p' 开启/关闭桌面系统代理。无需 root 权限，用户态即可控制。"),
        ]),
    ];

    let sysproxy_card = Paragraph::new(sysproxy_card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(" 2. 桌面系统代理权限 (GSettings) "),
    );
    f.render_widget(sysproxy_card, chunks[2]);

    // 4. Action & Instructions
    let footer_text = vec![
        Line::from(vec![
            Span::styled("快捷操作指南:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" • 按 'P' 键: ", Style::default().fg(Color::Green)),
            Span::raw("触发 Linux Polkit 图形化/终端密码验证窗口，自动为 Mihomo 核心赋予 cap_net_admin 权限"),
        ]),
        Line::from(vec![
            Span::styled(" • 按 'X' 键: ", Style::default().fg(Color::Cyan)),
            Span::raw("开启 / 关闭 TUN 虚拟网卡模式"),
        ]),
    ];

    let footer_p = Paragraph::new(footer_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" 操作提示 "),
        );
    f.render_widget(footer_p, chunks[3]);
}
