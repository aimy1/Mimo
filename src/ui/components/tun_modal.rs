use crate::app::AppState;
use crate::ui::i18n::Language;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);
    let popup_area = centered_rect(70, 62, area);
    f.render_widget(Clear, popup_area);

    let border_color = if state.is_granting_privilege {
        Color::Yellow
    } else if state.is_tun_privileged {
        Color::Green
    } else {
        Color::Cyan
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
        .title(title_text);

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header description
            Constraint::Length(3), // Target Binary Path
            Constraint::Length(3), // Password Input Box
            Constraint::Length(3), // Action Buttons
            Constraint::Min(1),    // Footer Status & Help
        ])
        .split(inner_area);

    // 1. Header description
    let header_lines = if lang == Language::Zh {
        vec![
            Line::from(vec![
                Span::styled("🛡️ 权限用途说明: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("开启 TUN 模式需创建 "),
                Span::styled("tun0", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" 虚拟网卡以接管全局网络流量。"),
            ]),
            Line::from(vec![
                Span::raw("   系统将为 Mihomo 核心二进制文件赋予 "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" Capability 权限。"),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("🛡️ Permission Purpose: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("TUN mode creates virtual interface "),
                Span::styled("tun0", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" to manage IP traffic."),
            ]),
            Line::from(vec![
                Span::raw("   Grants "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" capability to Mihomo core binary."),
            ]),
        ]
    };

    let header_p = Paragraph::new(header_lines)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header_p, chunks[0]);

    // 2. Target Binary Path
    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "未知/未找到路径".to_string());

    let target_lines = vec![Line::from(vec![
        Span::styled("🎯 授权目标程序: ", Style::default().fg(Color::Cyan)),
        Span::styled(binary_path, Style::default().fg(Color::White)),
    ])];

    let target_p = Paragraph::new(target_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" 核心路径 (Core Path) "),
    );
    f.render_widget(target_p, chunks[1]);

    // 3. Sudo Password Input Box
    let is_pass_focused = state.tun_input_focus == 0;
    let pass_border_style = if is_pass_focused {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let pass_display = if state.tun_password_input.is_empty() {
        if is_pass_focused {
            Span::styled("🔑 请在此输入 Linux Sudo / Root 密码 (按 Enter 提交)...", Style::default().fg(Color::DarkGray))
        } else {
            Span::styled("🔑 点击或按 Tab 聚焦输入 Sudo 密码", Style::default().fg(Color::DarkGray))
        }
    } else {
        let masked = "*".repeat(state.tun_password_input.chars().count());
        Span::styled(masked, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    };

    let pass_p = Paragraph::new(vec![Line::from(vec![
        Span::raw(" "),
        pass_display,
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(pass_border_style)
            .title(" Sudo / Root 密码 (Password) "),
    );
    f.render_widget(pass_p, chunks[2]);

    // 4. Action Buttons
    let action_widget = if state.is_granting_privilege {
        let loading_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" ⏳ 正在验证系统密码并赋予 CAP_NET_ADMIN 权限，请稍候... ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" ⏳ Verifying password and granting CAP_NET_ADMIN capability... ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ])]
        };
        Paragraph::new(loading_text).alignment(Alignment::Center)
    } else {
        let btn1_style = if state.tun_input_focus == 0 || state.tun_input_focus == 1 {
            Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(Color::DarkGray)
        };
        let btn2_style = if state.tun_input_focus == 2 {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };

        let btn_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" [ Enter : Sudo 密码授权 ] ", btn1_style),
                Span::raw("  "),
                Span::styled(" [ P 键 : 尝试 Desktop Polkit GUI 窗口 ] ", btn2_style),
                Span::raw("  "),
                Span::styled(" [ Esc : 取消 ] ", Style::default().fg(Color::White).bg(Color::Red)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" [ Enter : Submit Sudo Password ] ", btn1_style),
                Span::raw("  "),
                Span::styled(" [ Key P : Polkit GUI Dialog ] ", btn2_style),
                Span::raw("  "),
                Span::styled(" [ Esc : Cancel ] ", Style::default().fg(Color::White).bg(Color::Red)),
            ])]
        };
        Paragraph::new(btn_text).alignment(Alignment::Center)
    };

    f.render_widget(action_widget, chunks[3]);

    // 5. Footer Status & Hints
    let footer_text = if lang == Language::Zh {
        vec![
            Line::from(Span::styled("提示 1: 输入密码时系统将在后台安全提交，不会在控制台或界面明文显示", Style::default().fg(Color::DarkGray))),
            Line::from(Span::styled("提示 2: 若桌面运行了 GNOME/KDE/XFCE 等 GUI 环境，可按 'P' 键拉起 Polkit 密码弹窗", Style::default().fg(Color::DarkGray))),
        ]
    } else {
        vec![
            Line::from(Span::styled("Tip 1: Password is safely handled via stdin and masked on UI", Style::default().fg(Color::DarkGray))),
            Line::from(Span::styled("Tip 2: Press 'P' to try desktop Polkit GUI authentication window", Style::default().fg(Color::DarkGray))),
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
