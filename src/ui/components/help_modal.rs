use crate::ui::i18n::{t, Language};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, lang_str: &str) {
    let lang = Language::from_str(lang_str);
    let popup_area = centered_rect(65, 70, area);
    f.render_widget(Clear, popup_area);

    let help_text = if lang == Language::Zh {
        vec![
            Line::from(Span::styled("Mimo 焦点驱动键盘与鼠标指南 (Lazygit-Style)", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Esc               ", Style::default().fg(Color::Yellow)),
                Span::raw(": 关闭弹窗/取消搜索 | 从工作区切回左侧菜单焦点"),
            ]),
            Line::from(vec![
                Span::styled("  h / l / ← / →     ", Style::default().fg(Color::Yellow)),
                Span::raw(": 在 菜单 ↔ 工作区 或 代理组 ↔ 节点列表 间切换焦点"),
            ]),
            Line::from(vec![
                Span::styled("  j / k / ↑ / ↓     ", Style::default().fg(Color::Yellow)),
                Span::raw(": 上下移动列表/表格焦点"),
            ]),
            Line::from(vec![
                Span::styled("  g / G             ", Style::default().fg(Color::Yellow)),
                Span::raw(": 一键跳至列表 顶部(g) 或 底部(G)"),
            ]),
            Line::from(vec![
                Span::styled("  Tab / 1-9         ", Style::default().fg(Color::Yellow)),
                Span::raw(": 顺序切换 / 数字直达 9 大页面 (含 9-权限管理)"),
            ]),
            Line::from(vec![
                Span::styled("  Enter             ", Style::default().fg(Color::Yellow)),
                Span::raw(": 确认选择节点 / 激活订阅 / 保存设置"),
            ]),
            Line::from(vec![
                Span::styled("  t                 ", Style::default().fg(Color::Yellow)),
                Span::raw(": 测试当前代理组所有节点延迟"),
            ]),
            Line::from(vec![
                Span::styled("  m / p / x / r     ", Style::default().fg(Color::Yellow)),
                Span::raw(": 模式(m) | 系统代理(p) | TUN(x) | 重启核心(r)"),
            ]),
            Line::from(vec![
                Span::styled("  a / u / d / D     ", Style::default().fg(Color::Yellow)),
                Span::raw(": 添加(a) | 更新(u) | 删除(d) | 切断全部连接(Shift+D)"),
            ]),
            Line::from(vec![
                Span::styled("  s 或 /            ", Style::default().fg(Color::Yellow)),
                Span::raw(": 唤出/关闭节点实时Searching搜索框"),
            ]),
            Line::from(vec![
                Span::styled("  q / Ctrl+C        ", Style::default().fg(Color::Yellow)),
                Span::raw(": 退出 Mimo"),
            ]),
            Line::from(""),
            Line::from(Span::styled("按 'Esc' 或 '?' 关闭", Style::default().fg(Color::DarkGray))),
        ]
    } else {
        vec![
            Line::from(Span::styled("Mimo Focus-Driven Keyboard Guide (Lazygit-Style)", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Esc               ", Style::default().fg(Color::Yellow)),
                Span::raw(": Close modal/search | Switch focus back to Sidebar"),
            ]),
            Line::from(vec![
                Span::styled("  h / l / ← / →     ", Style::default().fg(Color::Yellow)),
                Span::raw(": Switch focus between Sidebar ↔ Workspace / Groups ↔ Nodes"),
            ]),
            Line::from(vec![
                Span::styled("  j / k / ↑ / ↓     ", Style::default().fg(Color::Yellow)),
                Span::raw(": Move selection down / up in focused list"),
            ]),
            Line::from(vec![
                Span::styled("  g / G             ", Style::default().fg(Color::Yellow)),
                Span::raw(": Jump to top (g) or bottom (G) of list"),
            ]),
            Line::from(vec![
                Span::styled("  Tab / 1-8         ", Style::default().fg(Color::Yellow)),
                Span::raw(": Cycle tabs or jump directly to tab 1-8"),
            ]),
            Line::from(vec![
                Span::styled("  Enter             ", Style::default().fg(Color::Yellow)),
                Span::raw(": Select node / Activate profile / Save settings"),
            ]),
            Line::from(vec![
                Span::styled("  t                 ", Style::default().fg(Color::Yellow)),
                Span::raw(": Test latency for current group nodes"),
            ]),
            Line::from(vec![
                Span::styled("  m / p / x / r     ", Style::default().fg(Color::Yellow)),
                Span::raw(": Cycle Mode (m) | SysProxy (p) | TUN (x) | Restart Core (r)"),
            ]),
            Line::from(vec![
                Span::styled("  a / u / d / D     ", Style::default().fg(Color::Yellow)),
                Span::raw(": Add profile (a) | Update (u) | Delete (d) | Close all conns (D)"),
            ]),
            Line::from(vec![
                Span::styled("  s or /            ", Style::default().fg(Color::Yellow)),
                Span::raw(": Toggle node searching input bar"),
            ]),
            Line::from(vec![
                Span::styled("  q / Ctrl+C        ", Style::default().fg(Color::Yellow)),
                Span::raw(": Quit Mimo"),
            ]),
            Line::from(""),
            Line::from(Span::styled("Press 'Esc' or '?' to close", Style::default().fg(Color::DarkGray))),
        ]
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
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
