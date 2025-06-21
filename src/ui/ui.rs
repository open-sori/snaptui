use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
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
    server_version: &str,
) -> Result<()> {
    // Use the last_message field
    let message = app_state.last_message.lock().unwrap();
    let last_message = message.as_str();

    terminal.draw(|f| {
        let size = f.size();
        let layout = create_layout(size);

        draw_header(f, app_state, layout[0]);
        draw_message_panel(f, app_state, last_message, layout[1]);
        draw_footer(f, app_state, server_version, layout[2]);
    })?;
    Ok(())
}

fn create_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Header
            Constraint::Min(1),       // Main content
            Constraint::Length(3),    // Footer
        ].as_ref())
        .split(area)
        .to_vec()
}

fn draw_header(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_guard = app_state.status.lock().unwrap();
    let (status_text, status_color) = get_status_info(&status_guard);

    let header_text = Paragraph::new(format!("snaptui | {}", status_text))
        .style(Style::default().fg(status_color))
        .alignment(Alignment::Center);

    f.render_widget(header_text, area);
}

fn draw_footer(f: &mut Frame, _app_state: &AppState, server_version: &str, area: Rect) {
    let footer_block = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let footer_content = if server_version.is_empty() {
        "Press 'q' to quit".to_string()
    } else {
        format!("Snapcast Server: v{} | Press 'q' to quit", server_version)
    };

    let footer_text = Paragraph::new(footer_content)
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    f.render_widget(footer_block, area);
    f.render_widget(footer_text, Rect {
        x: 1,
        y: 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    });
}

fn draw_message_panel(f: &mut Frame, _app_state: &AppState, last_message: &str, area: Rect) {
    let messages_block = Block::default()
        .title("Information")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));

    let info_text = Paragraph::new(last_message)
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(messages_block, area);
    f.render_widget(info_text, Rect {
        x: 1,
        y: 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    });
}

fn get_status_info(status: &ConnectionStatus) -> (&str, Color) {
    match status {
        ConnectionStatus::Connected => ("Connected", Color::Green),
        ConnectionStatus::Disconnected => ("Disconnected", Color::Red),
        ConnectionStatus::Connecting => ("Connecting...", Color::Yellow),
        ConnectionStatus::Error(e) => (e.as_str(), Color::Red),
    }
}