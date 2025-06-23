mod status;
mod events;  // Changed from infos to events

use ratatui::{
    prelude::*,
    layout::{Layout, Constraint, Direction, Rect},
};
use crate::ui::AppState;

pub fn draw_footer(f: &mut Frame, app_state: &AppState, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Events
            Constraint::Percentage(30), // Status
        ])
        .split(area);

    events::draw_events(f, app_state, layout[0]);
    status::draw_status(f, app_state, layout[1]);
}