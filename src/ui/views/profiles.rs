use crate::app::AppState;
use ratatui::{layout::Rect, Frame};

pub fn render(f: &mut Frame, state: &mut AppState, area: Rect) {
    crate::ui::profile::render(f, state, area);
}

