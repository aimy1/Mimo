use crate::app::AppState;
use ratatui::{layout::Rect, Frame};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    crate::ui::proxy::render(f, state, area);
}

