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

    let (zone_str, zone_style) = match state.focus_zone {
        FocusZone::Sidebar => (
            match lang { Language::Zh => " 侧边栏 ", Language::En => " SIDEBAR " },
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
        ),
        FocusZone::Workspace => (
            match lang { Language::Zh => " 工作区 ", Language::En => " MAIN " },
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::SECONDARY).add_modifier(Modifier::BOLD),
        ),
    };

    let (hint_nav, hint_pane, hint_move, hint_mode, hint_proxy, hint_tun, hint_help, hint_quit) = match lang {
        Language::Zh => ("页面", "分栏", "导航", "模式", "代理", "TUN", "帮助", "退出"),
        Language::En => ("Tabs", "Pane", "Nav", "Mode", "Proxy", "TUN", "Help", "Quit"),
    };

    let key_style = Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD);
    let desc_style = Style::default().fg(Theme::TEXT_MUTED);

    let mut spans = vec![
        Span::raw(" "),
        Span::styled(zone_str, zone_style),
        Span::raw("  "),
        Span::styled("1-0 ", key_style),
        Span::styled(format!("{} ", hint_nav), desc_style),
        Span::styled("Tab/hl ", key_style),
        Span::styled(format!("{} ", hint_pane), desc_style),
        Span::styled("jk ", key_style),
        Span::styled(format!("{} ", hint_move), desc_style),
        Span::styled("m ", key_style),
        Span::styled(format!("{} ", hint_mode), desc_style),
        Span::styled("p ", key_style),
        Span::styled(format!("{} ", hint_proxy), desc_style),
        Span::styled("x ", key_style),
        Span::styled(format!("{} ", hint_tun), desc_style),
        Span::styled("? ", key_style),
        Span::styled(format!("{} ", hint_help), desc_style),
        Span::styled("q ", key_style),
        Span::styled(hint_quit, desc_style),
    ];

    if let Some((toast_msg, _)) = &state.toast {
        spans.push(Span::raw("   "));
        spans.push(Span::styled(
            format!(" 🔔 {} ", toast_msg),
            Style::default().fg(Color::Rgb(17, 17, 27)).bg(Theme::WARN_YELLOW).add_modifier(Modifier::BOLD),
        ));
    }

    let paragraph = Paragraph::new(Line::from(spans));
    f.render_widget(paragraph, area);
}

