mod list;
mod details;

use ratatui::{
    prelude::*,
    layout::{Rect},
};
use crate::ui::AppState;

pub fn draw_client_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    list::draw_client_list(f, app_state, area);
}

pub fn draw_client_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    details::draw_client_details(f, app_state, area);
}
