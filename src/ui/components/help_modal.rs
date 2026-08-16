use crate::ui::i18n::{t, Language};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, lang_str: &str) {
    let lang = Language::from_str(lang_str);
    let popup_area = centered_rect(65, 72, area);
    f.render_widget(Clear, popup_area);

    let key_style = Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD);

    let help_text = if lang == Language::Zh {
        vec![
            Line::from(Span::styled("Mimo 键盘与交互指南 (Lazygit-Style)", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Esc               ", key_style),
                Span::styled(": 关闭弹窗/取消搜索 | 从工作区切回侧边栏", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  h / l / ← / →     ", key_style),
                Span::styled(": 在 菜单 ↔ 工作区 或 分组 ↔ 节点 间切换焦点", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  j / k / ↑ / ↓     ", key_style),
                Span::styled(": 上下逐行移动列表/表格焦点", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  Ctrl+d / Ctrl+u   ", key_style),
                Span::styled(": 列表半页快速翻页 (PageDown / PageUp)", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  g / G             ", key_style),
                Span::styled(": 一键跳至列表 顶部(g) 或 底部(G)", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  Tab / 1-9 / 0     ", key_style),
                Span::styled(": 切换/直达 10 大主功能页面", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  Enter             ", key_style),
                Span::styled(": 确认选择节点 / 激活订阅 / 保存设置", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  t                 ", key_style),
                Span::styled(": 测试当前代理组所有节点 / 常用网站延迟", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  m / p / x / r     ", key_style),
                Span::styled(": 模式(m) | 系统代理(p) | TUN(x) | 重启核心(r)", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  a / u / d / D     ", key_style),
                Span::styled(": 添加(a) | 更新(u) | 删除(d) | 切断全部连接(Shift+D)", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  s 或 /            ", key_style),
                Span::styled(": 唤出/关闭实时搜索过滤框", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  q / Ctrl+C        ", key_style),
                Span::styled(": 退出 Mimo", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(""),
            Line::from(Span::styled("按 'Esc' 或 '?' 关闭帮助", Style::default().fg(Theme::TEXT_MUTED))),
        ]
    } else {
        vec![
            Line::from(Span::styled("Mimo Focus-Driven Keyboard Guide", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Esc               ", key_style),
                Span::styled(": Close modal/search | Switch focus back to Sidebar", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  h / l / ← / →     ", key_style),
                Span::styled(": Switch focus between Sidebar ↔ Workspace / Groups ↔ Nodes", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  j / k / ↑ / ↓     ", key_style),
                Span::styled(": Move selection down / up in focused list", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  g / G             ", key_style),
                Span::styled(": Jump to top (g) or bottom (G) of list", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  Tab / 1-0         ", key_style),
                Span::styled(": Cycle tabs or jump directly to tab 1-0", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  Enter             ", key_style),
                Span::styled(": Select node / Activate profile / Save settings", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  t                 ", key_style),
                Span::styled(": Test latency for current group nodes", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  m / p / x / r     ", key_style),
                Span::styled(": Cycle Mode (m) | SysProxy (p) | TUN (x) | Restart Core (r)", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  a / u / d / D     ", key_style),
                Span::styled(": Add profile (a) | Update (u) | Delete (d) | Close conns (D)", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  s or /            ", key_style),
                Span::styled(": Toggle search filter bar", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(vec![
                Span::styled("  q / Ctrl+C        ", key_style),
                Span::styled(": Quit Mimo", Style::default().fg(Theme::TEXT_MAIN)),
            ]),
            Line::from(""),
            Line::from(Span::styled("Press 'Esc' or '?' to close", Style::default().fg(Theme::TEXT_MUTED))),
        ]
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::PRIMARY))
        .title(format!(" {} ", t("footer_help", lang)));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, popup_area);
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

