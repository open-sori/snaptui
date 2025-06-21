use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color},
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Result};
use std::sync::{Arc, Mutex};
use crate::websocket::ConnectionStatus;

pub struct AppState {
    pub last_message: Arc<Mutex<String>>,
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
        let layout = create_layout(size);

        draw_message_panel(f, app_state, layout[0]);
        draw_status_panel(f, app_state, layout[1]);
    })?;  // Added the ? operator to properly handle the Result

    Ok(())
}

fn create_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ].as_ref())
        .split(area)
        .to_vec()
}

fn draw_message_panel(f: &mut Frame, app_state: &AppState, area: Rect) {
    let messages_block = Block::default()
        .title("Last Snapcast Message")
        .borders(Borders::ALL);
    f.render_widget(messages_block, area);

    let last_message = app_state.last_message.lock().unwrap();
    let paragraph = Paragraph::new(last_message.as_str())
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(paragraph, Rect {
        x: 1,
        y: 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
        });
}

fn draw_status_panel(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_block = Block::default()
        .title("Connection Status")
        .borders(Borders::ALL);
    f.render_widget(status_block, area);

        let status_guard = app_state.status.lock().unwrap();
    let (status_text, status_color) = get_status_info(&status_guard);

    let status_paragraph = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(status_color));
    f.render_widget(status_paragraph, Rect {
        x: 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
        });
}

fn get_status_info(status: &ConnectionStatus) -> (&str, Color) {
    match status {
        ConnectionStatus::Connected => ("Connected", Color::Green),
        ConnectionStatus::Disconnected => ("Disconnected", Color::Yellow),
        ConnectionStatus::Connecting => ("Connecting...", Color::Yellow),
        ConnectionStatus::Error(e) => (e.as_str(), Color::Red),
    }
}