mod list;
mod details;

use ratatui::{
    prelude::*,
    layout::{Layout, Constraint, Direction, Rect},
};
use crate::ui::AppState;

pub fn draw_clients(f: &mut Frame, app_state: &AppState, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // List
            Constraint::Percentage(70), // Details
        ])
        .split(area);

    list::draw_client_list(f, app_state, layout[0]);
    details::draw_client_details(f, app_state, layout[1]);
}