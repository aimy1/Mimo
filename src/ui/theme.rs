use ratatui::style::{Color, Modifier, Style};

#[allow(dead_code)]
pub struct Theme;

impl Theme {
    // Catppuccin Mocha & Modern Tokyo Night Theme Palette
    pub const BORDER: Color = Color::Rgb(88, 91, 112); // Muted Surface Slate
    pub const BORDER_FOCUS: Color = Color::Rgb(203, 166, 247); // Electric Lavender / Purple
    pub const MODE_BADGE: Color = Color::Rgb(250, 179, 135); // Warm Peach / Gold
    pub const ACTIVE_GREEN: Color = Color::Rgb(166, 227, 161); // Emerald Green
    pub const TEXT_MUTED: Color = Color::Rgb(108, 112, 134);

    pub const SIDEBAR_SELECTED: Style = Style::new()
        .fg(Color::Rgb(17, 17, 27))
        .bg(Color::Rgb(203, 166, 247))
        .add_modifier(Modifier::BOLD);

    pub const ITEM_SELECTED: Style = Style::new()
        .fg(Color::Rgb(17, 17, 27))
        .bg(Color::Rgb(137, 220, 235)) // Soft Cyan
        .add_modifier(Modifier::BOLD);

    // Latency Pill Styles
    pub const PILL_GOOD: Style = Style::new()
        .fg(Color::Rgb(17, 17, 27))
        .bg(Color::Rgb(166, 227, 161))
        .add_modifier(Modifier::BOLD);

    pub const PILL_MEDIUM: Style = Style::new()
        .fg(Color::Rgb(17, 17, 27))
        .bg(Color::Rgb(249, 226, 175))
        .add_modifier(Modifier::BOLD);

    pub const PILL_BAD: Style = Style::new()
        .fg(Color::White)
        .bg(Color::Rgb(243, 139, 168))
        .add_modifier(Modifier::BOLD);

    pub const PILL_UNTESTED: Style = Style::new().fg(Color::Rgb(108, 112, 134));

    pub const TRAFFIC_UP: Color = Color::Rgb(137, 220, 235); // Sky Blue
    pub const TRAFFIC_DOWN: Color = Color::Rgb(166, 227, 161); // Emerald Green
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_speed(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}
