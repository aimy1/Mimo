use crate::app::state::{AppState, FocusZone};
use crate::ui::i18n::{t, Language};
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let focus_tag = match state.focus_zone {
        FocusZone::Sidebar => Span::styled(" [焦点: 菜单] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)),
        FocusZone::Workspace => Span::styled(" [焦点: 工作区] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
    };

    let mut spans = vec![
        focus_tag,
        Span::raw(" "),
        Span::styled("Tab/1-8", Style::default().fg(Color::Yellow)),
        Span::styled(format!(":{} ", t("footer_switch_tab", lang)), Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(":返回侧边栏 "),
        Span::styled("h/l/←/→", Style::default().fg(Color::Yellow)),
        Span::raw(":切换分栏 "),
        Span::styled("j/k/↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(":移动 "),
        Span::styled("g/G", Style::default().fg(Color::Yellow)),
        Span::raw(":顶/底 "),
        Span::styled("m", Style::default().fg(Color::Yellow)),
        Span::raw(":模式 "),
        Span::styled("p", Style::default().fg(Color::Yellow)),
        Span::raw(":系统代理 "),
        Span::styled("x", Style::default().fg(Color::Yellow)),
        Span::raw(":TUN "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(":帮助 "),
    ];

    if let Some((toast_msg, _)) = &state.toast {
        spans.push(Span::styled(
            format!(" | 🔔 {}", toast_msg),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ));
    }

    let paragraph = Paragraph::new(Line::from(spans));
    f.render_widget(paragraph, area);
}
