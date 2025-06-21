mod status;
mod events;

use ratatui::{
    prelude::*,
    layout::{Layout, Constraint, Direction, Rect},
};
use crate::ui::AppState;

pub fn draw_footer(f: &mut Frame, app_state: &AppState, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(85),
            Constraint::Percentage(15),
        ])
        .split(area);

    events::draw_events(f, app_state, layout[0]);
    status::draw_status(f, app_state, layout[1]);
}