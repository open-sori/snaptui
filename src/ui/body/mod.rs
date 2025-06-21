pub mod groups;
pub mod clients;
pub mod streams;

pub mod tabs;

use ratatui::{
    prelude::*,
    layout::{Layout, Constraint, Direction, Rect},
};
use crate::ui::AppState;

pub use tabs::TabSelection;
pub use groups::GroupDetailsFocus;
pub use clients::ClientDetailsFocus;

pub fn draw_body(f: &mut Frame, app_state: &AppState, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);
    draw_left_column(f, app_state, layout[0]);
    draw_right_column(f, app_state, layout[1]);
}

fn draw_left_column(f: &mut Frame, app_state: &AppState, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Min(1),
        ])
        .split(area);

    tabs::draw_tabs(f, app_state, layout[0]);

    let active_tab = app_state.active_tab.lock().unwrap();
    match *active_tab {
        TabSelection::Groups => groups::draw_group_list(f, app_state, layout[1]),
        TabSelection::Clients => clients::draw_client_list(f, app_state, layout[1]),
        TabSelection::Streams => streams::draw_stream_list(f, app_state, layout[1]),
    }
}

fn draw_right_column(f: &mut Frame, app_state: &AppState, area: Rect) {
    let active_tab = app_state.active_tab.lock().unwrap();
    match *active_tab {
        TabSelection::Groups => groups::draw_group_details(f, app_state, area),
        TabSelection::Clients => clients::draw_client_details(f, app_state, area),
        TabSelection::Streams => streams::draw_stream_details(f, app_state, area),
    }
}