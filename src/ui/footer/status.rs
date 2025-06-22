use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Rect, Alignment},
    style::{Style, Color},
};
use crate::ui::AppState;
use crate::websocket::ConnectionStatus;

pub fn draw_status(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status = app_state.status.lock().unwrap();
    let version = app_state.server_version.lock().unwrap();

    let status_text = match *status {
        ConnectionStatus::Connected => format!("Snapcast v{} | Connected", version),
        ConnectionStatus::Disconnected => "Disconnected".to_string(),
        ConnectionStatus::Connecting => "Connecting...".to_string(),
        ConnectionStatus::Error(_) => "Error".to_string(),
    };

    let status_color = match *status {
        ConnectionStatus::Connected => Color::Green,
        ConnectionStatus::Disconnected => Color::Red,
        ConnectionStatus::Connecting => Color::Yellow,
        ConnectionStatus::Error(_) => Color::Red,
    };

    let status_block = Paragraph::new(status_text)
        .style(Style::default().fg(status_color))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" [ Status ] "));

    f.render_widget(status_block, area);
}