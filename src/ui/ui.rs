use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Rect},
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Result};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::websocket::ConnectionStatus;

pub struct AppState {
    pub messages: Arc<Mutex<VecDeque<String>>>,
    pub status: Arc<Mutex<ConnectionStatus>>,
}

pub fn initialize_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app_state: &AppState,
) -> Result<()> {
    terminal.draw(|f| {
        let size = f.size();

        // Create layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
            ].as_ref())
            .split(size);

        // Messages panel
        let messages_block = Block::default()
            .title("Snapcast Messages")
            .borders(Borders::ALL);
        f.render_widget(messages_block, layout[0]);

        let messages = app_state.messages.lock().unwrap();
        let text = messages.iter().fold(String::new(), |acc, s| acc + s + "\n");
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(paragraph, Rect {
            x: 1,
            y: 1,
            width: layout[0].width.saturating_sub(2),
            height: layout[0].height.saturating_sub(2),
        });

        // Status panel
        let status_block = Block::default()
            .title("Connection Status")
            .borders(Borders::ALL);
        f.render_widget(status_block, layout[1]);

        let status_guard = app_state.status.lock().unwrap();
        let status_text = match *status_guard {
            ConnectionStatus::Connected => "Connected",
            ConnectionStatus::Disconnected => "Disconnected",
            ConnectionStatus::Connecting => "Connecting...",
            ConnectionStatus::Error(ref e) => e,
        };

        let status_paragraph = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(status_paragraph, Rect {
            x: 1,
            y: layout[1].y + 1,
            width: layout[1].width.saturating_sub(2),
            height: layout[1].height.saturating_sub(2),
        });
    })?; // Add the ? operator to properly handle the Result

    Ok(())
}