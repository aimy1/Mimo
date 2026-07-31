use crate::ui::i18n::{t, Language};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, lang_str: &str) {
    let lang = Language::from_str(lang_str);
    let popup_area = centered_rect(65, 65, area);
    f.render_widget(Clear, popup_area);

    let help_text = if lang == Language::Zh {
        vec![
            Line::from(Span::styled("Mimo 快捷键与鼠标指南", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Tab / 1-7     ", Style::default().fg(Color::Yellow)),
                Span::raw(": 切换 Tab 页面 (仪表盘/节点/订阅/连接/流量/日志/设置)"),
            ]),
            Line::from(vec![
                Span::styled("  鼠标点击 / 滚轮 ", Style::default().fg(Color::Yellow)),
                Span::raw(": 点击顶部切换 Tab | 点击选择节点/订阅 | 滚轮上下滚动"),
            ]),
            Line::from(vec![
                Span::styled("  j / k / ↑ / ↓ ", Style::default().fg(Color::Yellow)),
                Span::raw(": 上下移动焦点 / 滚动列表"),
            ]),
            Line::from(vec![
                Span::styled("  h / l / ← / → ", Style::default().fg(Color::Yellow)),
                Span::raw(": 切换节点分栏 (代理组/节点列表)"),
            ]),
            Line::from(vec![
                Span::styled("  Enter         ", Style::default().fg(Color::Yellow)),
                Span::raw(": 确认选择节点 / 激活订阅"),
            ]),
            Line::from(vec![
                Span::styled("  t             ", Style::default().fg(Color::Yellow)),
                Span::raw(": 测试当前代理组所有节点延迟"),
            ]),
            Line::from(vec![
                Span::styled("  m             ", Style::default().fg(Color::Yellow)),
                Span::raw(": 循环切换模式 (规则 -> 全局 -> 直连)"),
            ]),
            Line::from(vec![
                Span::styled("  p / x / r     ", Style::default().fg(Color::Yellow)),
                Span::raw(": 开关系统代理 (p) | 开关 TUN (x) | 重启内核 (r)"),
            ]),
            Line::from(vec![
                Span::styled("  d             ", Style::default().fg(Color::Yellow)),
                Span::raw(": 断开选定连接 / 删除订阅配置"),
            ]),
            Line::from(vec![
                Span::styled("  ?             ", Style::default().fg(Color::Yellow)),
                Span::raw(": 打开/关闭此帮助对话框"),
            ]),
            Line::from(vec![
                Span::styled("  q / Ctrl+C    ", Style::default().fg(Color::Yellow)),
                Span::raw(": 退出 Mimo"),
            ]),
            Line::from(""),
            Line::from(Span::styled("按 'Esc' 或 '?' 关闭", Style::default().fg(Color::DarkGray))),
        ]
    } else {
        vec![
            Line::from(Span::styled("Mimo Keybindings & Mouse Guide", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Tab / 1-7     ", Style::default().fg(Color::Yellow)),
                Span::raw(": Switch tabs (Dashboard/Proxies/Profiles/Conns/Traffic/Logs/Settings)"),
            ]),
            Line::from(vec![
                Span::styled("  Mouse Click / Wheel ", Style::default().fg(Color::Yellow)),
                Span::raw(": Click header tabs | Click to select | Mouse Wheel scroll"),
            ]),
            Line::from(vec![
                Span::styled("  j / k / ↑ / ↓ ", Style::default().fg(Color::Yellow)),
                Span::raw(": Navigate items / scroll list"),
            ]),
            Line::from(vec![
                Span::styled("  h / l / ← / → ", Style::default().fg(Color::Yellow)),
                Span::raw(": Switch pane focus in Proxies view"),
            ]),
            Line::from(vec![
                Span::styled("  Enter         ", Style::default().fg(Color::Yellow)),
                Span::raw(": Confirm node selection / Activate profile"),
            ]),
            Line::from(vec![
                Span::styled("  t             ", Style::default().fg(Color::Yellow)),
                Span::raw(": Test latency for current group nodes"),
            ]),
            Line::from(vec![
                Span::styled("  m             ", Style::default().fg(Color::Yellow)),
                Span::raw(": Cycle operating mode (Rule -> Global -> Direct)"),
            ]),
            Line::from(vec![
                Span::styled("  p / x / r     ", Style::default().fg(Color::Yellow)),
                Span::raw(": SysProxy (p) | TUN (x) | Restart Core (r)"),
            ]),
            Line::from(vec![
                Span::styled("  d             ", Style::default().fg(Color::Yellow)),
                Span::raw(": Close connection / Delete profile"),
            ]),
            Line::from(vec![
                Span::styled("  ?             ", Style::default().fg(Color::Yellow)),
                Span::raw(": Toggle this help dialog"),
            ]),
            Line::from(vec![
                Span::styled("  q / Ctrl+C    ", Style::default().fg(Color::Yellow)),
                Span::raw(": Quit Mimo"),
            ]),
            Line::from(""),
            Line::from(Span::styled("Press 'Esc' or '?' to close", Style::default().fg(Color::DarkGray))),
        ]
    };

    let block = Block::default()
        .borders(Borders::ALL)
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
