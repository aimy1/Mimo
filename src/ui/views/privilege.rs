use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

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
    let header_title = match lang {
        Language::Zh => " 🔒 系统权限管理与提权控制 ",
        Language::En => " 🔒 System Privilege & Authorization Control Center ",
    };

    let header_text = vec![
        Line::from(vec![
            Span::styled(header_title, Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
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
        let text = match lang { Language::Zh => " [ ✔ 已授权 ] ", Language::En => " [ ✔ Authorized ] " };
        Span::styled(text, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        let text = match lang { Language::Zh => " [ ✖ 未授权 ] ", Language::En => " [ ✖ Unauthorized ] " };
        Span::styled(text, Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD))
    };

    let tun_state_badge = if state.is_tun_enabled {
        let text = match lang { Language::Zh => " [ ● TUN: 已开启 ] ", Language::En => " [ ● TUN: ENABLED ] " };
        Span::styled(text, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        let text = match lang { Language::Zh => " [ ○ TUN: 已关闭 ] ", Language::En => " [ ○ TUN: DISABLED ] " };
        Span::styled(text, Style::default().fg(Color::White).bg(Color::Rgb(88, 91, 112)))
    };

    let iface_badge = if state.is_tun_interface_up {
        Span::styled(format!(" [ 🌐 {} (UP) ] ", state.tun_interface_name), Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD))
    } else if state.tun_interface_name != "None" {
        Span::styled(format!(" [ 🌐 {} (DOWN) ] ", state.tun_interface_name), Style::default().fg(Color::Rgb(249, 226, 175)))
    } else {
        let text = match lang { Language::Zh => " [ 🌐 未创建 ] ", Language::En => " [ 🌐 Not Created ] " };
        Span::styled(text, Style::default().fg(Theme::TEXT_MUTED))
    };

    let not_found_str = match lang { Language::Zh => "未知路径", Language::En => "Unknown path" };
    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| not_found_str.to_string());

    let (label_name, label_status, label_core, btn_grant, btn_revoke, btn_tun, title_card1) = match lang {
        Language::Zh => ("权限名称: ", "当前状态: ", "核心程序: ", " [ 🔑 授权 Root (按 P 键) ] ", " [ 🚫 撤销权限 (按 R 键) ] ", " [ ⚡ TUN 开关 (按 X 键) ] ", " 1. TUN 虚拟网卡提权 (CAP_NET_ADMIN) "),
        Language::En => ("Capability : ", "Status     : ", "Core Path  : ", " [ 🔑 Grant Root (P) ] ", " [ 🚫 Revoke (R) ] ", " [ ⚡ Toggle TUN (X) ] ", " 1. TUN Virtual Adapter Privilege (CAP_NET_ADMIN) "),
    };

    let tun_card_text = vec![
        Line::from(vec![
            Span::styled(label_name, Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::raw("Linux CAP_NET_ADMIN (TUN Virtual Network Interface)"),
        ]),
        Line::from(vec![
            Span::styled(label_status, Style::default().fg(Color::Rgb(137, 220, 235))),
            auth_badge,
            Span::raw("  "),
            tun_state_badge,
            Span::raw("  "),
            iface_badge,
        ]),
        Line::from(vec![
            Span::styled(label_core, Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::styled(binary_path, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![]),
        Line::from(vec![
            Span::styled(btn_grant, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
            Span::raw("   "),
            Span::styled(btn_revoke, Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD)),
            Span::raw("   "),
            Span::styled(btn_tun, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let tun_card = Paragraph::new(tun_card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if is_auth { Theme::ACTIVE_GREEN } else { Color::Rgb(243, 139, 168) }))
            .title(title_card1),
    );
    f.render_widget(tun_card, chunks[1]);

    // 3. System Proxy GSettings Card
    let (g_name, g_desc, title_card2) = match lang {
        Language::Zh => ("GNOME Desktop GSettings 系统代理修改权限", "允许按 'p' 开启/关闭桌面系统代理 (HTTP/SOCKS 端口控制)", " 2. 桌面系统代理权限 (GSettings) "),
        Language::En => ("GNOME Desktop GSettings System Proxy Capability", "Allows toggling Desktop SysProxy via 'p' (HTTP/SOCKS ports)", " 2. System Proxy Capability (GSettings) "),
    };

    let sysproxy_card_text = vec![
        Line::from(vec![
            Span::styled(label_name, Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::raw(g_name),
            Span::raw("   Status: "),
            Span::styled(" [ ✔ READY ] ", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Purpose : ", Style::default().fg(Color::Rgb(137, 220, 235))),
            Span::raw(g_desc),
        ]),
    ];

    let sysproxy_card = Paragraph::new(sysproxy_card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_card2),
    );
    f.render_widget(sysproxy_card, chunks[2]);

    // 4. Action & Instructions
    let (guide_title, guide_content) = match lang {
        Language::Zh => (" 操作提示 Guide ", " [P] Sudo/Polkit 授权  [R] 撤销 setcap 权限  [X] TUN 开关 "),
        Language::En => (" Keybindings Guide ", " [P] Sudo/Polkit Grant  [R] Revoke setcap  [X] Toggle TUN "),
    };

    let footer_text = vec![
        Line::from(vec![
            Span::styled(" Keybindings: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(guide_content, Style::default().fg(Color::Rgb(205, 214, 244))),
        ]),
    ];

    let footer_p = Paragraph::new(footer_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(guide_title),
        );
    f.render_widget(footer_p, chunks[3]);
}
