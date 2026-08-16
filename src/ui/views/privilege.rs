use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // TUN privilege card
            Constraint::Length(5),  // GSettings card
            Constraint::Min(0),     // Instructions
        ])
        .split(area);

    // 1. TUN Privilege Card
    let is_auth = state.is_tun_privileged;
    let auth_badge = if is_auth {
        let text = match lang { Language::Zh => " 已授权 ", Language::En => " Authorized " };
        Span::styled(text, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        let text = match lang { Language::Zh => " 未授权 ", Language::En => " Unauthorized " };
        Span::styled(text, Style::default().fg(Color::White).bg(Theme::DANGER_RED).add_modifier(Modifier::BOLD))
    };

    let tun_state_badge = if state.is_tun_enabled {
        let text = match lang { Language::Zh => " TUN: 开启 ", Language::En => " TUN: ON " };
        Span::styled(text, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        let text = match lang { Language::Zh => " TUN: 关闭 ", Language::En => " TUN: OFF " };
        Span::styled(text, Style::default().fg(Theme::TEXT_MUTED).bg(Theme::BG_SURFACE))
    };

    let iface_badge = if state.is_tun_interface_up {
        Span::styled(format!(" {} (UP) ", state.tun_interface_name), Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::SECONDARY).add_modifier(Modifier::BOLD))
    } else if state.tun_interface_name != "None" {
        Span::styled(format!(" {} (DOWN) ", state.tun_interface_name), Style::default().fg(Theme::WARN_YELLOW))
    } else {
        let text = match lang { Language::Zh => " 未创建 ", Language::En => " Not Created " };
        Span::styled(text, Style::default().fg(Theme::TEXT_MUTED))
    };

    let not_found_str = match lang { Language::Zh => "未知路径", Language::En => "Unknown path" };
    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| not_found_str.to_string());

    let (label_name, label_status, label_core, btn_grant, btn_revoke, btn_tun, title_card1) = match lang {
        Language::Zh => ("权限能力: ", "当前状态: ", "核心路径: ", "P: 授权 Root", "R: 撤销权限", "X: TUN 开关", " 🔒 1. TUN 虚拟网卡提权 (CAP_NET_ADMIN) "),
        Language::En => ("Capability: ", "Status    : ", "Core Path : ", "P: Grant Root", "R: Revoke", "X: Toggle TUN", " 🔒 1. TUN Virtual Adapter Privilege (CAP_NET_ADMIN) "),
    };

    let tun_card_text = vec![
        Line::from(vec![
            Span::styled(format!(" {}", label_name), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Linux CAP_NET_ADMIN (TUN Virtual Network Interface)", Style::default().fg(Theme::TEXT_MAIN)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_status), Style::default().fg(Theme::TEXT_MUTED)),
            Span::raw(" "),
            auth_badge,
            Span::raw("  "),
            tun_state_badge,
            Span::raw("  "),
            iface_badge,
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_core), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(binary_path, Style::default().fg(Theme::TEXT_SUB)),
        ]),
        Line::from(vec![]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled(format!(" [{}] ", btn_grant), Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(format!(" [{}] ", btn_revoke), Style::default().fg(Theme::DANGER_RED).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(format!(" [{}] ", btn_tun), Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let tun_card = Paragraph::new(tun_card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if is_auth { Theme::ACTIVE_GREEN } else { Theme::WARN_YELLOW }))
            .title(title_card1),
    );
    f.render_widget(tun_card, chunks[0]);

    // 2. System Proxy GSettings Card
    let (g_name, g_desc, title_card2) = match lang {
        Language::Zh => ("GNOME GSettings 代理配置权限", "允许通过快捷键 'p' 开启/关闭桌面系统代理 (HTTP/SOCKS 端口)", " 🌐 2. 桌面系统代理设置 (GSettings) "),
        Language::En => ("GNOME GSettings Proxy Capability", "Allows toggling system proxy via 'p' (HTTP/SOCKS ports)", " 🌐 2. Desktop System Proxy (GSettings) "),
    };

    let sysproxy_card_text = vec![
        Line::from(vec![
            Span::styled(" 能力名称: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(g_name, Style::default().fg(Theme::TEXT_MAIN)),
            Span::raw("  "),
            Span::styled(" [ READY ] ", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 说明用途: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(g_desc, Style::default().fg(Theme::TEXT_SUB)),
        ]),
    ];

    let sysproxy_card = Paragraph::new(sysproxy_card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_card2),
    );
    f.render_widget(sysproxy_card, chunks[1]);
}

