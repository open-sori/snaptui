use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Padding},
    layout::{Rect, Alignment},
    style::{Style, Color},
};
use crate::ui::AppState;
use crate::websocket::ConnectionStatus;
use crate::ui::utils::apply_margin;

pub fn draw_status(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status = app_state.status.lock().unwrap();
    let version = app_state.server_version.lock().unwrap();
    let margin = 1;
    let display_version = if version.len() > 10 {
        format!("{}...", &version[..7])
    } else {
        version.clone()
    };

    let (status_text, status_color) = match *status {
        ConnectionStatus::Connected => {
            (format!("Snapcast v{} | Connected", display_version), Color::Green)
        }
        ConnectionStatus::Disconnected => ("Disconnected".to_string(), Color::Red),
        ConnectionStatus::Connecting => ("Connecting...".to_string(), Color::Yellow),
        ConnectionStatus::Error(ref e) => {
            let error_msg = if e.len() > 20 {
                format!("{}...", &e[..17])
            } else {
                e.clone()
            };
            (format!("Error: {}", error_msg), Color::Red)
        }
    };

    let border_color = match *status {
        ConnectionStatus::Connected => Color::Green,
        _ => Color::Red,
    };

    let status_block = Paragraph::new(status_text)
        .style(Style::default().fg(status_color).bold())
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" [ Status ] ")
            .border_style(Style::default().fg(border_color))
            .title_style(Style::default().fg(border_color))
            .style(Style::default().fg(Color::White))
            .padding(Padding::new(1, 1, 1, 1)))
        .style(Style::default().fg(status_color));

    let inner_area = apply_margin(area, margin);
    f.render_widget(status_block, inner_area);
}