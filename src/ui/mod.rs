mod headers;
mod tabs;
mod body;
mod footer;
mod utils;
pub use headers::draw_header;
pub use tabs::{draw_tabs, TabSelection, Tab};
pub use body::draw_body;
pub use footer::draw_footer;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Rect},
};
use std::sync::{Arc, Mutex};
use std::io::{self, Result};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crate::websocket::ConnectionStatus;
use crate::models::server::getstatus::GetStatusData;

pub struct AppState {
    pub last_message: Arc<Mutex<String>>,
    pub status: Arc<Mutex<ConnectionStatus>>,
    pub server_version: Arc<Mutex<String>>,
    pub status_data: Arc<Mutex<Option<GetStatusData>>>,
    pub active_tab: Arc<Mutex<TabSelection>>,
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
        draw_tabs(f, app_state, layout[1]);
        draw_body(f, app_state, layout[2]);
        draw_footer(f, app_state, layout[3]);
    })?;
    Ok(())
}

fn create_main_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Header
            Constraint::Length(3),    // Tabs
            Constraint::Min(1),       // Body
            Constraint::Length(3),    // Footer
        ])
        .split(area)
        .to_vec()
}