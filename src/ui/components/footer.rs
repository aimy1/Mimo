use crate::app::state::{AppState, FocusZone};
use crate::ui::i18n::Language;
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

    let focus_tag = match (state.focus_zone, lang) {
        (FocusZone::Sidebar, Language::Zh) => Span::styled(" [焦点: 侧边栏] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)),
        (FocusZone::Sidebar, Language::En) => Span::styled(" [Focus: Sidebar] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::BORDER_FOCUS).add_modifier(Modifier::BOLD)),
        (FocusZone::Workspace, Language::Zh) => Span::styled(" [焦点: 工作区] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        (FocusZone::Workspace, Language::En) => Span::styled(" [Focus: Workspace] ", Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
    };

    let (label_tab, label_esc, label_pane, label_nav, label_top, label_mode, label_sys, label_tun, label_help) = match lang {
        Language::Zh => (":切换页面 ", ":返回菜单 ", ":切换分栏 ", ":移动 ", ":顶/底 ", ":模式 ", ":系统代理 ", ":TUN ", ":帮助 "),
        Language::En => (":Tabs ", ":Menu ", ":Pane ", ":Move ", ":Top/Bot ", ":Mode ", ":SysProxy ", ":TUN ", ":Help "),
    };

    let mut spans = vec![
        focus_tag,
        Span::raw(" "),
        Span::styled("1-0", Style::default().fg(Color::Yellow)),
        Span::styled(label_tab, Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(label_esc),
        Span::styled("h/l", Style::default().fg(Color::Yellow)),
        Span::raw(label_pane),
        Span::styled("j/k", Style::default().fg(Color::Yellow)),
        Span::raw(label_nav),
        Span::styled("g/G", Style::default().fg(Color::Yellow)),
        Span::raw(label_top),
        Span::styled("m", Style::default().fg(Color::Yellow)),
        Span::raw(label_mode),
        Span::styled("p", Style::default().fg(Color::Yellow)),
        Span::raw(label_sys),
        Span::styled("x", Style::default().fg(Color::Yellow)),
        Span::raw(label_tun),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(label_help),
    ];

    if let Some((toast_msg, _)) = &state.toast {
        spans.push(Span::styled(
            format!(" [ 🔔 {} ] ", toast_msg),
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD),
        ));
    }

    let paragraph = Paragraph::new(Line::from(spans));
    f.render_widget(paragraph, area);
}
