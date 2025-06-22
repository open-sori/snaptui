use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Wrap, ListDirection},
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
use crate::models::server::getstatus::GetStatusData;

pub struct AppState {
    pub last_message: Arc<Mutex<String>>,
    pub status: Arc<Mutex<ConnectionStatus>>,
    pub server_version: Arc<Mutex<String>>,
    pub status_data: Arc<Mutex<Option<GetStatusData>>>,
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
        let area = f.area();
        let layout = create_layout(area);

        draw_header(f, app_state, layout[0]);
        draw_content(f, app_state, layout[1]);
        draw_footer(f, app_state, layout[2]);
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

fn draw_footer(f: &mut Frame, app_state: &AppState, area: Rect) {
    let version_guard = app_state.server_version.lock().unwrap();
    let version = version_guard.as_str();

    let footer_block = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let footer_content = if version.is_empty() {
        "Press 'q' to quit".to_string()
    } else {
        format!("Snapcast Server: v{} | Press 'q' to quit", version)
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

fn draw_content(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data_guard = app_state.status_data.lock().unwrap();

    let content_block = Block::default()
        .title("Streams Information")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));

    f.render_widget(content_block, area);

    let inner_area = Rect {
        x: 1,
        y: 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    if let Some(status) = &*status_data_guard {
        draw_stream_info(f, status, inner_area);
    } else {
        draw_waiting_message(f, app_state, inner_area);
    }
}

fn draw_stream_info(f: &mut Frame, status: &GetStatusData, area: Rect) {
    let mut items = Vec::new();

    // Add basic server info
    items.push(ListItem::new(format!(
        "Server: {} | Version: {}",
        status.result.server.server.host.name,
        status.result.server.server.snapserver.version
    )));

        // Add stream count information
    items.push(ListItem::new(""));
    items.push(ListItem::new(format!(
        "Total Streams: {}",
        status.result.server.streams.len()
    )));

    // Add stream details
    for stream in &status.result.server.streams {
        items.push(ListItem::new(format!("Stream ID: {}", stream.id)));
        items.push(ListItem::new(format!("  Status: {}", stream.status)));
        items.push(ListItem::new(format!("  URI: {}", stream.uri.raw)));
        items.push(ListItem::new(""));
    }

    let list = List::new(items)
        .style(Style::default().fg(Color::White))
        .direction(ListDirection::TopToBottom);
    f.render_widget(list, area);
}

fn draw_waiting_message(f: &mut Frame, app_state: &AppState, area: Rect) {
    let message_guard = app_state.last_message.lock().unwrap();
    let last_message = message_guard.as_str();

    let info_text = Paragraph::new(last_message)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(info_text, area);
}

fn get_status_info(status: &ConnectionStatus) -> (&str, Color) {
    match status {
        ConnectionStatus::Connected => ("Connected", Color::Green),
        ConnectionStatus::Disconnected => ("Disconnected", Color::Red),
        ConnectionStatus::Connecting => ("Connecting...", Color::Yellow),
        ConnectionStatus::Error(e) => (e.as_str(), Color::Red),
    }
}