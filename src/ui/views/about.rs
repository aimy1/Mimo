use crate::app::AppState;
use crate::ui::i18n::Language;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
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
            Span::styled("   __  __ _                    ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  |  \\/  (_)_ __ ___   ___     ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  | |\\/| | | '_ ` _ \\ / _ \\    ", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  | |  | | | | | | | | (_) |   ", Style::default().fg(Theme::ACTIVE_GREEN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  |_|  |_|_|_| |_| |_|\\___/    ", Style::default().fg(Theme::WARN_YELLOW).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Minimalist Mihomo Manager ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
            Span::styled(concat!(" v", env!("CARGO_PKG_VERSION")), Style::default().fg(Theme::TEXT_MUTED)),
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
        .map(|v| format!("v{} ({})", v.version, v.meta))
        .unwrap_or_else(|| "Offline".into());

    let (label_name, label_author, label_ver, label_core, label_target, label_license, label_repo, title_meta) = match lang {
        Language::Zh => ("软件名称: ", "软件作者: ", "软件版本: ", "核心版本: ", "运行架构: ", "开源协议: ", "代码仓库: ", " 项目信息 "),
        Language::En => ("Name    : ", "Author  : ", "Version : ", "Core Ver: ", "Target  : ", "License : ", "GitHub  : ", " Project Info "),
    };

    let card1_text = vec![
        Line::from(vec![
            Span::styled(format!(" {}", label_name), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Mimo (Minimalist Mihomo Manager)", Style::default().fg(Theme::TEXT_MAIN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_author), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("aisaniya", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_ver), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(format!("v{} (Rust 2024)", env!("CARGO_PKG_VERSION")), Style::default().fg(Theme::ACTIVE_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_core), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(core_ver, Style::default().fg(Theme::WARN_YELLOW)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_target), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Linux x86_64 (Terminal TUI / CLI)", Style::default().fg(Theme::SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_license), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("MIT License", Style::default().fg(Theme::PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_repo), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("https://github.com/aimy1/Mimo", Style::default().fg(Theme::SECONDARY)),
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
        Language::Zh => ("开发语言: ", "终端引擎: ", "异步运行: ", "界面配色: ", " 技术架构 "),
        Language::En => ("Language: ", "TUI Core: ", "Async   : ", "Theme   : ", " Architecture "),
    };

    let card2_text = vec![
        Line::from(vec![
            Span::styled(format!(" {}", label_lang), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Rust 1.80+ (Zero-Cost Abstractions)", Style::default().fg(Theme::TEXT_MAIN)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_ui), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Ratatui v0.29 + Crossterm", Style::default().fg(Theme::SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_async), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Tokio + Reqwest + WebSockets", Style::default().fg(Theme::ACTIVE_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {}", label_theme), Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled("Catppuccin Mocha Dark Palette", Style::default().fg(Theme::PRIMARY)),
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
        Language::Zh => ("鸣谢开源社区:", " 致谢鸣谢 "),
        Language::En => ("Special Thanks:", " Acknowledgments "),
    };


    let card3_text = vec![
        Line::from(vec![
            Span::styled(format!(" {}", label_thanks), Style::default().fg(Theme::WARN_YELLOW).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  - Mihomo Team (MetaCubeX/mihomo)", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
        Line::from(vec![
            Span::styled("  - Ratatui Community (ratatui-org)", Style::default().fg(Theme::TEXT_MUTED)),
        ]),
        Line::from(vec![
            Span::styled("  - Catppuccin Theme Community", Style::default().fg(Theme::TEXT_MUTED)),
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

