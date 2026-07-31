use crate::app::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = crate::ui::i18n::Language::from_str(&state.settings_lang);
    let mut spans = vec![
        Span::styled(" Tab/1-7", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_nav", lang))),
        Span::styled("j/k/Scroll", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_scroll", lang))),
        Span::styled("Enter/Click", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_select", lang))),
        Span::styled("t", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_test", lang))),
        Span::styled("m", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_mode", lang))),
        Span::styled("p", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_sysproxy", lang))),
        Span::styled("x", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_tun", lang))),
        Span::styled("r", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_restart", lang))),
        Span::styled("?", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}  ", crate::ui::i18n::t("footer_help", lang))),
        Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(format!(":{}", crate::ui::i18n::t("footer_quit", lang))),
    ];

    if let Some((msg, _)) = &state.toast {
        spans.push(Span::raw(" | "));
        spans.push(Span::styled(
            format!(" {} ", msg),
            Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
        ));
    }

    let footer = Paragraph::new(Line::from(spans));
    f.render_widget(footer, area);
}
