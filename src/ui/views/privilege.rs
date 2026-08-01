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
        .constraints([
            Constraint::Length(3),  // Header description
            Constraint::Length(10), // TUN privilege card
            Constraint::Length(6),  // GSettings card
            Constraint::Min(3),     // Action & Instructions
        ])
        .split(area);

    // 1. Header description
    let header_text = vec![
        Line::from(vec![
            Span::styled(" 🔒 系统权限管理与提权控制 ", Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
            Span::styled(" (Linux CAP_NET_ADMIN & GSettings)", Style::default().fg(Theme::TEXT_MUTED)),
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
        Span::styled(" [ ✔ 已授权 ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [ ✖ 未授权 ] ", Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD))
    };

    let tun_state_badge = if state.is_tun_enabled {
        Span::styled(" [ ● TUN: 已开启 ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [ ○ TUN: 已关闭 ] ", Style::default().fg(Color::White).bg(Color::Rgb(88, 91, 112)))
    };

    let iface_badge = if state.is_tun_interface_up {
        Span::styled(format!(" [ 🌐 {} (UP) ] ", state.tun_interface_name), Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD))
    } else if state.tun_interface_name != "None" {
        Span::styled(format!(" [ 🌐 {} (DOWN) ] ", state.tun_interface_name), Style::default().fg(Color::Rgb(249, 226, 175)))
    } else {
        Span::styled(" [ 🌐 未创建 ] ", Style::default().fg(Theme::TEXT_MUTED))
    };

    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "未知路径".to_string());

    let tun_card_text = vec![
        Line::from(vec![
            Span::styled("权限名称: ", Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::raw("Linux CAP_NET_ADMIN (TUN 虚拟网卡全局路由)"),
        ]),
        Line::from(vec![
            Span::styled("当前状态: ", Style::default().fg(Color::Rgb(137, 220, 235))),
            auth_badge,
            Span::raw("  "),
            tun_state_badge,
            Span::raw("  "),
            iface_badge,
        ]),
        Line::from(vec![
            Span::styled("核心程序: ", Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::styled(binary_path, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![]),
        Line::from(vec![
            Span::styled(" [ 🔑 授权 Root (按 P 键) ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
            Span::raw("   "),
            Span::styled(" [ 🚫 撤销权限 (按 R 键) ] ", Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD)),
            Span::raw("   "),
            Span::styled(" [ ⚡ TUN 开关 (按 X 键) ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let tun_card = Paragraph::new(tun_card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if is_auth { Theme::ACTIVE_GREEN } else { Color::Rgb(243, 139, 168) }))
            .title(" 1. TUN 虚拟网卡提权 (CAP_NET_ADMIN) "),
    );
    f.render_widget(tun_card, chunks[1]);

    // 3. System Proxy GSettings Card
    let sysproxy_card_text = vec![
        Line::from(vec![
            Span::styled("权限名称: ", Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::raw("GNOME Desktop GSettings 系统代理修改权限"),
            Span::raw("   状态: "),
            Span::styled(" [ ✔ READY ] ", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("作用说明: ", Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::raw("允许按 'p' 开启/关闭桌面系统代理 (HTTP/SOCKS 端口控制)"),
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
            Span::styled(" 快捷按键: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(" [P] Sudo/Polkit 授权  [R] 撤销 setcap 权限  [X] TUN 开关 ", Style::default().fg(Color::Rgb(205, 214, 244))),
        ]),
    ];

    let footer_p = Paragraph::new(footer_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(" 操作提示 Guide "),
        );
    f.render_widget(footer_p, chunks[3]);
}
