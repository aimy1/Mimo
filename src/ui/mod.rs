pub mod components;
pub mod i18n;
pub mod profile;
pub mod proxy;
pub mod theme;
pub mod views;

use crate::app::{AppState, Tab};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState) {
    let root_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(18), // Left Sidebar Navigation
            Constraint::Min(0),     // Right Workspace Area
        ])
        .split(f.area());

    // 1. Render Left Sidebar Navigation
    components::sidebar::render(f, state, root_layout[0]);

    // Right workspace vertical layout
    let workspace_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Control Pills Bar
            Constraint::Min(0),    // Main Content View
            Constraint::Length(1), // Footer Keyhints
        ])
        .split(root_layout[1]);

    // 2. Top Control Pills Bar
    components::header::render(f, state, workspace_layout[0]);

    // 3. Active Tab View
    match state.active_tab {
        Tab::Dashboard => views::dashboard::render(f, state, workspace_layout[1]),
        Tab::Proxies => proxy::render(f, state, workspace_layout[1]),
        Tab::Profiles => profile::render(f, state, workspace_layout[1]),
        Tab::Rules => views::rules::render(f, state, workspace_layout[1]),
        Tab::Connections => views::connections::render(f, state, workspace_layout[1]),
        Tab::Traffic => views::traffic::render(f, state, workspace_layout[1]),
        Tab::Logs => views::logs::render(f, state, workspace_layout[1]),
        Tab::Settings => views::settings::render(f, state, workspace_layout[1]),
        Tab::Privileges => views::privilege::render(f, state, workspace_layout[1]),
    }

    // 4. Footer
    components::footer::render(f, state, workspace_layout[2]);

    // 5. Input Modal Overlay for Subscription Add
    if state.show_profile_input {
        components::input_modal::render(f, state, f.area());
    }

    // 6. Help Modal Overlay if active
    if state.show_help {
        components::help_modal::render(f, f.area(), &state.settings_lang);
    }
}
