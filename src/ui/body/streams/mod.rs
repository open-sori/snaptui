mod list;
mod details;

use ratatui::{
    prelude::*,
    layout::{Rect},
};
use crate::ui::AppState;

pub fn draw_stream_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    list::draw_stream_list(f, app_state, area);
}

pub fn draw_stream_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    details::draw_stream_details(f, app_state, area);
}