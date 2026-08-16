use ratatui::style::{Color, Modifier, Style};

#[allow(dead_code)]
pub struct Theme;

impl Theme {
    // Catppuccin Mocha Palette
    pub const BG_BASE: Color = Color::Rgb(30, 30, 46);      // #1E1E2E
    pub const BG_SURFACE: Color = Color::Rgb(49, 50, 68);   // #313244
    pub const BG_OVERLAY: Color = Color::Rgb(69, 71, 90);   // #45475A

    pub const BORDER: Color = Color::Rgb(69, 71, 90);       // Subtle overlay border
    pub const BORDER_FOCUS: Color = Color::Rgb(203, 166, 247); // Mauve (Lavender)
    pub const BORDER_SUBTLE: Color = Color::Rgb(49, 50, 68);

    pub const TEXT_MAIN: Color = Color::Rgb(205, 214, 244); // #CDD6F4
    pub const TEXT_SUB: Color = Color::Rgb(186, 194, 222);  // #BAC2DE
    pub const TEXT_MUTED: Color = Color::Rgb(108, 112, 134); // #6C7086
    pub const TEXT_DIM: Color = Color::Rgb(88, 91, 112);     // #585B70

    pub const PRIMARY: Color = Color::Rgb(203, 166, 247);   // Mauve
    pub const SECONDARY: Color = Color::Rgb(137, 220, 235); // Sky
    pub const ACTIVE_GREEN: Color = Color::Rgb(166, 227, 161); // Green
    pub const MODE_BADGE: Color = Color::Rgb(250, 179, 135);  // Peach
    pub const WARN_YELLOW: Color = Color::Rgb(249, 226, 175); // Yellow
    pub const DANGER_RED: Color = Color::Rgb(243, 139, 168);  // Red / Flamingo

    pub const SIDEBAR_SELECTED: Style = Style::new()
        .fg(Color::Rgb(17, 17, 27))
        .bg(Color::Rgb(203, 166, 247))
        .add_modifier(Modifier::BOLD);

    pub const ITEM_SELECTED: Style = Style::new()
        .fg(Color::Rgb(17, 17, 27))
        .bg(Color::Rgb(137, 220, 235))
        .add_modifier(Modifier::BOLD);

    // Latency Pill Styles (Clean text with subtle tint)
    pub const PILL_GOOD: Style = Style::new()
        .fg(Color::Rgb(166, 227, 161))
        .add_modifier(Modifier::BOLD);

    pub const PILL_MEDIUM: Style = Style::new()
        .fg(Color::Rgb(249, 226, 175))
        .add_modifier(Modifier::BOLD);

    pub const PILL_BAD: Style = Style::new()
        .fg(Color::Rgb(243, 139, 168))
        .add_modifier(Modifier::BOLD);

    pub const PILL_UNTESTED: Style = Style::new()
        .fg(Color::Rgb(108, 112, 134));

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

