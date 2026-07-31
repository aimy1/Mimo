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
    let popup_area = centered_rect(66, 52, area);
    f.render_widget(Clear, popup_area);

    let border_color = if state.is_granting_privilege {
        Color::Yellow
    } else if state.is_tun_privileged {
        Color::Green
    } else {
        Color::Cyan
    };

    let title_text = if lang == Language::Zh {
        " 🛡️ TUN 虚拟网卡模式提权授权 "
    } else {
        " 🛡️ TUN Mode Privilege Escalation "
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
            Constraint::Length(4), // Header / Purpose
            Constraint::Length(4), // Target info
            Constraint::Length(3), // Action / Loading Box
            Constraint::Min(1),    // Hints / Footer
        ])
        .split(inner_area);

    // 1. Header / Purpose
    let header_lines = if lang == Language::Zh {
        vec![
            Line::from(vec![
                Span::styled("🔒 权限说明: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("TUN 模式需要在系统网络栈中创建 "),
                Span::styled("tun0", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" 虚拟网卡以接管全局 IP 流量。"),
            ]),
            Line::from(vec![
                Span::raw("   此操作需要授权 Linux 内核 "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" 提权。确定后将调起 Linux Polkit 系统授权窗口。"),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("🔒 Permission Notice: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("TUN mode creates virtual network interface "),
                Span::styled("tun0", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("."),
            ]),
            Line::from(vec![
                Span::raw("   Requires "),
                Span::styled("CAP_NET_ADMIN", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" capability via Linux Polkit authentication dialog."),
            ]),
        ]
    };

    let header_p = Paragraph::new(header_lines)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header_p, chunks[0]);

    // 2. Target info
    let binary_path = crate::core::CoreProcess::find_mihomo_binary()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "未知/未找到路径".to_string());

    let target_lines = vec![
        Line::from(vec![
            Span::styled("🎯 核心程序路径: ", Style::default().fg(Color::Cyan)),
            Span::styled(binary_path, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("⚡ 系统提权指令: ", Style::default().fg(Color::Cyan)),
            Span::styled("pkexec setcap cap_net_admin+ep <path>", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let target_p = Paragraph::new(target_lines)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" 授权详情 "),
        );
    f.render_widget(target_p, chunks[1]);

    // 3. Action / Loading Box
    let action_widget = if state.is_granting_privilege {
        let loading_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" ⏳ 正在等待 Linux Polkit 系统窗口输入密码授权... ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" ⏳ Waiting for Linux Polkit password authorization... ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ])]
        };
        Paragraph::new(loading_text).alignment(Alignment::Center)
    } else {
        let btn_text = if lang == Language::Zh {
            vec![Line::from(vec![
                Span::styled(" [ Enter / Y : 唤起提权授权窗口 ] ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled(" [ Esc / N : 取消 ] ", Style::default().fg(Color::White).bg(Color::Red)),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled(" [ Enter / Y : Grant Privileges ] ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled(" [ Esc / N : Cancel ] ", Style::default().fg(Color::White).bg(Color::Red)),
            ])]
        };
        Paragraph::new(btn_text).alignment(Alignment::Center)
    };

    f.render_widget(action_widget, chunks[2]);

    // 4. Hints / Footer
    let footer_text = if lang == Language::Zh {
        "提示: 系统授权窗口可能出现在桌面最前方，请在弹窗中输入 sudo/root 密码"
    } else {
        "Note: Polkit authentication window may appear on top of screen"
    };

    let footer_p = Paragraph::new(vec![Line::from(Span::styled(
        footer_text,
        Style::default().fg(Color::DarkGray),
    ))])
    .alignment(Alignment::Center);
    f.render_widget(footer_p, chunks[3]);
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
