use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Rect, Alignment, Layout, Direction, Constraint},
    style::{Style, Color},
};
use crate::ui::AppState;
use crate::core::websocket::connection::ConnectionStatus;
use crate::ui::utils::apply_margin;

pub fn draw_status(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status = app_state.status.lock().unwrap();
    let version = app_state.server_version.lock().unwrap();
    let margin = 1;

    let (line1_text, line2_text, status_color) = match *status {
        ConnectionStatus::Connected => (
            format!("Snapcast v{}", version),
            "Connected".to_string(),
            Color::Green,
        ),
        ConnectionStatus::Disconnected => (
            "Snapcast".to_string(),
            "Disconnected".to_string(),
            Color::Red,
        ),
        ConnectionStatus::Connecting => (
            "Snapcast".to_string(),
            "Connecting...".to_string(),
            Color::Yellow,
        ),
        ConnectionStatus::Error(ref e) => {
            let error_msg = if e.len() > 20 {
                format!("{}...", &e[..17])
            } else {
                e.clone()
            };
            ("Error".to_string(), error_msg, Color::Red)
        }
    };

    let border_color = match *status {
        ConnectionStatus::Connected => Color::Green,
        _ => Color::Red,
    };

    let line1_paragraph = Paragraph::new(line1_text)
        .style(Style::default().fg(status_color).bold())
        .alignment(Alignment::Center);

    let line2_paragraph = Paragraph::new(line2_text)
        .style(Style::default().fg(status_color).bold())
        .alignment(Alignment::Center);

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title(" [ Status ] ")
        .border_style(Style::default().fg(border_color))
        .title_style(Style::default().fg(border_color));

    let outer_area = apply_margin(area, margin);
    let inner_area = outer_block.inner(outer_area);
    f.render_widget(outer_block, outer_area);

    let text_block_height = 2; // The total height of our two lines of text

    // Layout to vertically center the 2-line text block
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // Top flexible spacer
            Constraint::Length(text_block_height), // Area for the two lines
            Constraint::Min(0), // Bottom flexible spacer
        ])
        .split(inner_area);

    let text_area = vertical_chunks[1]; // This is the vertically centered 2-line area

    // Layout to place the two text lines within their dedicated area
    let text_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Line 1
            Constraint::Length(1), // Line 2
        ])
        .split(text_area);

    f.render_widget(line1_paragraph, text_chunks[0]);
    f.render_widget(line2_paragraph, text_chunks[1]);
}