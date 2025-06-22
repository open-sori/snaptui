mod groups;
mod clients;
mod streams;

use ratatui::{
    prelude::*,
    layout::{Rect},
};
use crate::ui::AppState;
use super::TabSelection;

pub fn draw_body(f: &mut Frame, app_state: &AppState, area: Rect) {
    let active_tab = app_state.active_tab.lock().unwrap();

    match *active_tab {
        TabSelection::Groups => groups::draw_groups(f, app_state, area),
        TabSelection::Clients => clients::draw_clients(f, app_state, area),
        TabSelection::Streams => streams::draw_streams(f, app_state, area),
    }
}