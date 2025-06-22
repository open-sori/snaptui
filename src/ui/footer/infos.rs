use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Rect, Alignment},
    style::{Style, Color},
};
use crate::ui::AppState;

pub fn draw_infos(f: &mut Frame, app_state: &AppState, area: Rect) {
    let message = app_state.last_message.lock().unwrap();

    // Extract just the last part of the message if it contains multiple lines
    let display_message = if message.contains('\n') {
        message.lines().last().unwrap_or(&message).to_string()
    } else {
        message.clone()
    };

    // Truncate the message if it's too long for the panel
    let max_length = area.width as usize - 4; // Account for borders and padding
    let display_message = if display_message.len() > max_length {
        format!("{}...", &display_message[..max_length.saturating_sub(3)])
    } else {
        display_message
    };

    let info_block = Paragraph::new(display_message)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" [ Events ] ")
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White));

    f.render_widget(info_block, area);
}