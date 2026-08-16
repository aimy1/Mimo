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
    let popup_area = centered_rect(70, 60, area);
    f.render_widget(Clear, popup_area);

    let border_color = if state.is_granting_privilege {
        Theme::WARN_YELLOW
    } else if state.is_tun_privileged {
        Theme::ACTIVE_GREEN
    } else {
        Theme::PRIMARY
    };

    let title_text = if lang == Language::Zh {
        " TUN 虚拟网卡提权授权 "
    } else {
        " TUN Privilege Authorization "
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
        let text = match lang { Language::Zh => " 已授权 CAP_NET_ADMIN ", Language::En => " Authorized CAP_NET_ADMIN " };
        Span::styled(text, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    } else {
        let text = match lang { Language::Zh => " 未授权 CAP_NET_ADMIN ", Language::En => " Unauthorized CAP_NET_ADMIN " };
        Span::styled(text, Style::default().fg(Color::White).bg(Theme::DANGER_RED).add_modifier(Modifier::BOLD))
    };

    let iface_badge = if state.is_tun_interface_up {
        let text = match lang { Language::Zh => format!(" 网卡: {} (UP) ", state.tun_interface_name), Language::En => format!(" Interface: {} (UP) ", state.tun_interface_name) };
        Span::styled(text, Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::SECONDARY).add_modifier(Modifier::BOLD))
    } else {
        let text = match lang { Language::Zh => " 网卡: 未创建 ", Language::En => " Interface: None " };
        Span::styled(text, Style::default().fg(Theme::TEXT_MUTED).bg(Theme::BG_SURFACE))
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
                Span::styled("说明: ", Style::default().fg(Theme::TEXT_MUTED)),
                Span::raw("TUN 模式接管系统全局流量需创建虚拟网卡，需要 Linux "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
                Span::raw(" 提权。"),
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
                Span::styled("Note  : ", Style::default().fg(Theme::TEXT_MUTED)),
                Span::raw("TUN mode requires "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
                Span::raw(" capability to manage IP traffic."),
            ]),
        ]
    };

    let header_p = Paragraph::new(header_lines)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header_p, chunks[0]);

    // 2. Target Binary Path Card
    let not_found_str = match lang { Language::Zh => "未知路径", Language::En => "Unknown path" };
    let target_label = match lang { Language::Zh => " 🎯 目标核心: ", Language::En => " 🎯 Core Path: " };

    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| not_found_str.to_string());

    let target_lines = vec![Line::from(vec![
        Span::styled(target_label, Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled(binary_path, Style::default().fg(Theme::TEXT_SUB)),
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

    let pass_placeholder_active = match lang {
        Language::Zh => " 🔑 请在此输入 Sudo / Root 密码 (按 Enter 提交)_",
        Language::En => " 🔑 Enter Sudo / Root Password (Press Enter)_",
    };
    let pass_placeholder_idle = match lang {
        Language::Zh => " 🔑 请输入 Sudo 密码",
        Language::En => " 🔑 Enter Sudo password",
    };

    let pass_display = if state.tun_password_input.is_empty() {
        if is_pass_focused {
            Span::styled(pass_placeholder_active, Style::default().fg(Theme::TEXT_MUTED))
        } else {
            Span::styled(pass_placeholder_idle, Style::default().fg(Theme::TEXT_MUTED))
        }
    } else {
        let masked = "●".repeat(state.tun_password_input.chars().count());
        let cursor_suffix = if is_pass_focused { "_" } else { "" };
        Span::styled(format!(" {}{}", masked, cursor_suffix), Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD))
    };

    let title_pass_block = match lang {
        Language::Zh => " Sudo / Root 密码 ",
        Language::En => " Sudo / Root Password ",
    };

    let pass_p = Paragraph::new(vec![Line::from(vec![pass_display])]).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(pass_border_style)
            .title(title_pass_block),
    );
    f.render_widget(pass_p, chunks[2]);

    // 4. Action Pill Buttons
    let action_widget = if state.is_granting_privilege {
        let loading_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" ⏳ 正在验证密码并设置 CAP_NET_ADMIN 权限，请稍候... ", Style::default().fg(Theme::WARN_YELLOW).add_modifier(Modifier::BOLD)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" ⏳ Verifying password and granting capability... ", Style::default().fg(Theme::WARN_YELLOW).add_modifier(Modifier::BOLD)),
            ])]
        };
        Paragraph::new(loading_text).alignment(Alignment::Center)
    } else {
        let btn1_style = if state.tun_input_focus == 0 || state.tun_input_focus == 1 {
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::ACTIVE_GREEN).bg(Theme::BG_SURFACE)
        };
        let btn2_style = if state.tun_input_focus == 2 {
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::SECONDARY).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::SECONDARY).bg(Theme::BG_SURFACE)
        };

        let btn_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" [ Enter: 密码授权 ] ", btn1_style),
                Span::raw("  "),
                Span::styled(" [ P 键: Polkit GUI 窗口 ] ", btn2_style),
                Span::raw("  "),
                Span::styled(" [ Esc: 取消 ] ", Style::default().fg(Theme::DANGER_RED)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" [ Enter: Sudo Auth ] ", btn1_style),
                Span::raw("  "),
                Span::styled(" [ P: Polkit Dialog ] ", btn2_style),
                Span::raw("  "),
                Span::styled(" [ Esc: Cancel ] ", Style::default().fg(Theme::DANGER_RED)),
            ])]
        };
        Paragraph::new(btn_text).alignment(Alignment::Center)
    };

    f.render_widget(action_widget, chunks[3]);

    // 5. Footer Instructions & Security Tips
    let footer_text = if lang == Language::Zh {
        vec![
            Line::from(Span::styled("💡 密码仅用于本次授权，不会在任何地方保存", Style::default().fg(Theme::TEXT_MUTED))),
        ]
    } else {
        vec![
            Line::from(Span::styled("💡 Password is used only for current grant and never stored", Style::default().fg(Theme::TEXT_MUTED))),
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

