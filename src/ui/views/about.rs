use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let lang = Language::from_str(&state.settings_lang);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // ASCII Art Logo Banner
            Constraint::Min(0),    // Grid Cards
        ])
        .split(area);

    // 1. ASCII Art Logo Banner
    let logo_text = vec![
        Line::from(vec![
            Span::styled("   __  __ _                    ", Style::default().fg(Color::Rgb(203, 166, 247)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  |  \\/  (_)_ __ ___   ___     ", Style::default().fg(Color::Rgb(180, 190, 254)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  | |\\/| | | '_ ` _ \\ / _ \\    ", Style::default().fg(Color::Rgb(137, 220, 235)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  | |  | | | | | | | | (_) |   ", Style::default().fg(Color::Rgb(166, 227, 161)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  |_|  |_|_|_| |_| |_|\\___/    ", Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ⚡ Minimalist Mihomo Manager ", Style::default().fg(Color::Rgb(245, 194, 231)).add_modifier(Modifier::BOLD)),
            Span::styled(" v0.1.0", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
    ];

    let logo_block = Paragraph::new(logo_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER)),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(logo_block, chunks[0]);

    // 2. Main Grid Cards Layout
    let grid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55),
            Constraint::Percentage(45),
        ])
        .split(grid_chunks[1]);

    // Left Card: Project Metadata
    let core_ver = state
        .version
        .as_ref()
        .map(|v| format!("v{} (Meta: {})", v.version, v.meta))
        .unwrap_or_else(|| "Offline".into());

    let (label_name, label_author, label_ver, label_core, label_target, label_license, label_repo, title_meta) = match lang {
        Language::Zh => (" 软件名称 (Name)   : ", " 软件作者 (Author) : ", " 当前版本 (Version): ", " Mihomo 核心版本   : ", " 运行目标 (Target) : ", " 开源协议 (License): ", " 项目仓库 (GitHub) : ", " 📌 项目元数据 Metadata "),
        Language::En => (" Software Name     : ", " Software Author   : ", " Software Version  : ", " Mihomo Core Ver   : ", " Target Platform   : ", " License           : ", " GitHub Repository : ", " 📌 Project Metadata "),
    };

    let card1_text = vec![
        Line::from(vec![
            Span::styled(label_name, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Mimo (Minimalist Mihomo Manager)", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(label_author, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("aisaniya", Style::default().fg(Color::Rgb(243, 139, 168)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(label_ver, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("v0.1.0 (Rust 2021 Edition)", Style::default().fg(Theme::ACTIVE_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(label_core, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(core_ver, Style::default().fg(Color::Rgb(249, 226, 175))),
        ]),
        Line::from(vec![
            Span::styled(label_target, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Linux x86_64 (Terminal TUI / CLI)", Style::default().fg(Color::Rgb(137, 220, 235))),
        ]),
        Line::from(vec![
            Span::styled(label_license, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("MIT License", Style::default().fg(Color::Rgb(203, 166, 247))),
        ]),
        Line::from(vec![
            Span::styled(label_repo, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("https://github.com/aimy1/Mimo", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let card1_block = Paragraph::new(card1_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_meta),
    );
    f.render_widget(card1_block, grid_chunks[0]);

    // Right Top Card: Tech Stack
    let (label_lang, label_ui, label_async, label_theme, title_arch) = match lang {
        Language::Zh => (" 编程语言 : ", " 终端引擎 : ", " 异步运行时: ", " 视觉主题 : ", " 🏗️ 核心架构与技术栈 Architecture "),
        Language::En => (" Language : ", " TUI Engine: ", " Async Runtime: ", " UI Theme: ", " 🏗️ Architecture & Tech Stack "),
    };

    let card2_text = vec![
        Line::from(vec![
            Span::styled(label_lang, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Rust 1.80+ (High Performance & Zero-Cost)", Style::default().fg(Color::Rgb(250, 179, 135))),
        ]),
        Line::from(vec![
            Span::styled(label_ui, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Ratatui v0.29 + Crossterm", Style::default().fg(Color::Rgb(137, 220, 235))),
        ]),
        Line::from(vec![
            Span::styled(label_async, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Tokio + Reqwest + WebSocket Stream", Style::default().fg(Color::Rgb(166, 227, 161))),
        ]),
        Line::from(vec![
            Span::styled(label_theme, Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Catppuccin Morandi Dark Palette", Style::default().fg(Color::Rgb(203, 166, 247))),
        ]),
    ];

    let card2_block = Paragraph::new(card2_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_arch),
    );
    f.render_widget(card2_block, right_chunks[0]);

    // Right Bottom Card: Credits
    let (label_thanks, title_credits) = match lang {
        Language::Zh => (" 特别鸣谢 (Credits):", " 💖 致谢 Acknowledgments "),
        Language::En => (" Special Thanks (Credits):", " 💖 Acknowledgments "),
    };

    let card3_text = vec![
        Line::from(vec![
            Span::styled(label_thanks, Style::default().fg(Color::Rgb(249, 226, 175)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" - Mihomo Team (MetaCubeX/mihomo)", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
        Line::from(vec![
            Span::styled(" - Ratatui Community (ratatui-org)", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
        Line::from(vec![
            Span::styled(" - Catppuccin Theme Palette", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
    ];

    let card3_block = Paragraph::new(card3_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(title_credits),
    );
    f.render_widget(card3_block, right_chunks[1]);
}
