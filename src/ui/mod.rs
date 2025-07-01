mod headers;
mod body;
mod footer;
pub mod utils;

pub use headers::draw_header;
pub use body::draw_body;
pub use footer::draw_footer;
pub use body::TabSelection;
pub use body::GroupDetailsFocus;
pub use body::ClientDetailsFocus;

use ratatui::{
    prelude::*,
    layout::{Layout, Constraint, Direction, Rect},
};
use std::sync::{Arc, Mutex};
use std::io::{self, Result};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crate::core::websocket::connection::ConnectionStatus;
use crate::models::server::getstatus::GetStatusData;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum PanelFocus {
    List,
    Details,
}

pub struct AppState {
    pub last_message: Arc<Mutex<String>>,
    pub status: Arc<Mutex<ConnectionStatus>>,
    pub server_version: Arc<Mutex<String>>,
    pub status_data: Arc<Mutex<Option<GetStatusData>>>,
    pub active_tab: Arc<Mutex<TabSelection>>,
    pub selected_index: Arc<Mutex<usize>>,
    pub focused_panel: Arc<Mutex<PanelFocus>>,
    pub group_focused_field: Arc<Mutex<GroupDetailsFocus>>,
    pub client_focused_field: Arc<Mutex<ClientDetailsFocus>>,
    pub is_editing_client_name: Arc<Mutex<bool>>,
    pub editing_client_name: Arc<Mutex<String>>,
    pub is_editing_client_volume: Arc<Mutex<bool>>,
    pub editing_client_volume: Arc<Mutex<String>>,
    pub is_editing_client_latency: Arc<Mutex<bool>>,
    pub editing_client_latency: Arc<Mutex<String>>,
    pub cursor_visible: Arc<Mutex<bool>>,
    pub last_cursor_toggle: Arc<Mutex<Instant>>,
}

pub fn initialize_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(io::stdout()))?)
}

pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app_state: &AppState,
) -> Result<()> {
    terminal.draw(|f| {
        let area = f.area();
        let layout = create_main_layout(area);

        draw_header(f, app_state, layout[0]);
        draw_body(f, app_state, layout[1]);
        draw_footer(f, app_state, layout[2]);
    })?;
    Ok(())
}

fn create_main_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(7),
        ])
        .split(area)
        .to_vec()
}