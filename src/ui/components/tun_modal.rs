use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);
    let popup_area = centered_rect(72, 65, area);
    f.render_widget(Clear, popup_area);

    let border_color = if state.is_granting_privilege {
        Color::Rgb(249, 226, 175) // Morandi Yellow
    } else if state.is_tun_privileged {
        Theme::ACTIVE_GREEN
    } else {
        Theme::BORDER_FOCUS
    };

    let title_text = if lang == Language::Zh {
        " 🔒 Linux 系统 TUN 虚拟网卡提权授权中心 "
    } else {
        " 🔒 Linux TUN Mode Privilege Authorization Center "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(title_text, Style::default().fg(border_color).add_modifier(Modifier::BOLD)));

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4), // Header Status Badges & Purpose
            Constraint::Length(3), // Target Binary Path Card
            Constraint::Length(3), // Password Input Box
            Constraint::Length(3), // Action Buttons
            Constraint::Min(2),    // Footer Instructions & Tips
        ])
        .split(inner_area);

    // 1. Header Status Badges & Purpose
    let auth_badge = if state.is_tun_privileged {
        Span::styled(" [ ✔ 已授权 CAP_NET_ADMIN ] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [ ✖ 未授权 CAP_NET_ADMIN ] ", Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD))
    };

    let iface_badge = if state.is_tun_interface_up {
        Span::styled(format!(" [ 🌐 网卡: {} (UP) ] ", state.tun_interface_name), Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [ 🌐 网卡: 未创建 ] ", Style::default().fg(Color::White).bg(Color::Rgb(88, 91, 112)))
    };

    let header_lines = if lang == Language::Zh {
        vec![
            Line::from(vec![
                Span::styled("状态: ", Style::default().fg(Theme::TEXT_MUTED)),
                auth_badge,
                Span::raw("   "),
                iface_badge,
            ]),
            Line::from(vec![
                Span::styled("用途: ", Style::default().fg(Theme::TEXT_MUTED)),
                Span::raw("TUN 模式需创建 "),
                Span::styled("tun0", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
                Span::raw(" 虚拟网卡接管全局 IP 协议栈流量，需要系统的 "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
                Span::raw(" 权限。"),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Theme::TEXT_MUTED)),
                auth_badge,
                Span::raw("   "),
                iface_badge,
            ]),
            Line::from(vec![
                Span::styled("Purpose: ", Style::default().fg(Theme::TEXT_MUTED)),
                Span::raw("TUN mode creates virtual interface "),
                Span::styled("tun0", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
                Span::raw(" to manage IP traffic with "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
                Span::raw("."),
            ]),
        ]
    };

    let header_p = Paragraph::new(header_lines)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header_p, chunks[0]);

    // 2. Target Binary Path Card
    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "未知/未找到路径".to_string());

    let target_lines = vec![Line::from(vec![
        Span::styled(" 🎯 目标程序: ", Style::default().fg(Color::Rgb(137, 220, 235))),
        Span::styled(binary_path, Style::default().fg(Color::White)),
    ])];

    let target_p = Paragraph::new(target_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(" Mihomo Core Path "),
    );
    f.render_widget(target_p, chunks[1]);

    // 3. Password Input Box with Active Cursor
    let is_pass_focused = state.tun_input_focus == 0;
    let pass_border_style = if is_pass_focused {
        Style::default().fg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let pass_display = if state.tun_password_input.is_empty() {
        if is_pass_focused {
            Span::styled(" 🔑 请在此输入 Sudo / Root 密码 (按 Enter 提交)_", Style::default().fg(Color::Rgb(147, 153, 178)))
        } else {
            Span::styled(" 🔑 点击或按 Tab 聚焦输入 Sudo 密码", Style::default().fg(Theme::TEXT_MUTED))
        }
    } else {
        let masked = "●".repeat(state.tun_password_input.chars().count());
        let cursor_suffix = if is_pass_focused { "_" } else { "" };
        Span::styled(format!(" {}{}", masked, cursor_suffix), Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    };

    let pass_p = Paragraph::new(vec![Line::from(vec![pass_display])]).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(pass_border_style)
            .title(" Sudo / Root 密码 (Password Input) "),
    );
    f.render_widget(pass_p, chunks[2]);

    // 4. Action Pill Buttons
    let action_widget = if state.is_granting_privilege {
        let loading_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" ⏳ 正在安全验证密码并赋予 CAP_NET_ADMIN 权限，请稍候... ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" ⏳ Verifying password and granting CAP_NET_ADMIN capability... ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
            ])]
        };
        Paragraph::new(loading_text).alignment(Alignment::Center)
    } else {
        let btn1_style = if state.tun_input_focus == 0 || state.tun_input_focus == 1 {
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(Color::Rgb(49, 50, 68))
        };
        let btn2_style = if state.tun_input_focus == 2 {
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(137, 220, 235)).bg(Color::Rgb(30, 30, 46))
        };

        let btn_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" [ Enter: Sudo 密码授权 ] ", btn1_style),
                Span::raw("  "),
                Span::styled(" [ P 键: Desktop Polkit GUI 窗口 ] ", btn2_style),
                Span::raw("  "),
                Span::styled(" [ Esc: 取消 ] ", Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" [ Enter: Submit Sudo Password ] ", btn1_style),
                Span::raw("  "),
                Span::styled(" [ Key P: Polkit GUI Dialog ] ", btn2_style),
                Span::raw("  "),
                Span::styled(" [ Esc: Cancel ] ", Style::default().fg(Color::White).bg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD)),
            ])]
        };
        Paragraph::new(btn_text).alignment(Alignment::Center)
    };

    f.render_widget(action_widget, chunks[3]);

    // 5. Footer Instructions & Security Tips
    let footer_text = if lang == Language::Zh {
        vec![
            Line::from(Span::styled("💡 提示 1: 输入密码时系统将在后台安全提交，不会在控制台明文保存", Style::default().fg(Theme::TEXT_MUTED))),
            Line::from(Span::styled("💡 提示 2: 若桌面运行 GNOME/KDE 等环境，也可直接按 'P' 键弹拉 Linux Polkit GUI 验证框", Style::default().fg(Theme::TEXT_MUTED))),
        ]
    } else {
        vec![
            Line::from(Span::styled("💡 Tip 1: Password is sent securely via stdin and masked on UI", Style::default().fg(Theme::TEXT_MUTED))),
            Line::from(Span::styled("💡 Tip 2: Press 'P' to try desktop Polkit GUI authentication window", Style::default().fg(Theme::TEXT_MUTED))),
        ]
    };

    let footer_p = Paragraph::new(footer_text).alignment(Alignment::Center);
    f.render_widget(footer_p, chunks[4]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
