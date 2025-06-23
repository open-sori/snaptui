use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
    layout::{Rect},
    style::{Style, Color},
};
use crate::ui::AppState;
use crate::ui::utils::apply_margin;
use chrono::NaiveDateTime;

pub fn draw_events(f: &mut Frame, app_state: &AppState, area: Rect) {
    let message = app_state.last_message.lock().unwrap();
    let margin = 1;

    // Create a list of events with the message
    let items: Vec<ListItem> = if message.starts_with("[Notification]") {
        vec![ListItem::new(message.clone())
            .style(Style::default().fg(Color::Yellow))]
    } else if let Some(timestamp) = extract_timestamp(&message) {
        vec![
            ListItem::new(format!("Last update: {}", timestamp))
                .style(Style::default().fg(Color::Green)),
            ListItem::new(message.clone())
                .style(Style::default().fg(Color::White)),
        ]
    } else {
        vec![ListItem::new(message.clone())
            .style(Style::default().fg(Color::White))]
    };

    let events_block = List::new(items)
                    .style(Style::default().fg(Color::White))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" [ Events ] ")
            .padding(Padding::new(2, 2, 1, 1))
            .title_style(Style::default().fg(Color::White)))
        .highlight_style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(events_block, inner_area);
}

// Helper function to extract timestamp from messages
fn extract_timestamp(message: &str) -> Option<String> {
    // Look for timestamp patterns in the message
    if let Some(start) = message.find("Last update: ") {
        let timestamp_str = &message[start + "Last update: ".len()..];
        if let Ok(_) = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S") {
            return Some(timestamp_str.to_string());
        }
    }

    // Try to parse other timestamp formats
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(message) {
        return Some(dt.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    None
}