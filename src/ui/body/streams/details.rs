use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
};
use crate::ui::{AppState, utils::apply_margin};

pub fn draw_stream_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let margin = 1;

    if let Some(data) = &*status_data {
        if data.result.server.streams.len() > *selected_index {
            let stream = &data.result.server.streams[*selected_index];

            let details = vec![
                ListItem::new(format!("ID: {}", stream.id)),
                ListItem::new(format!("Status: {}", stream.status)),
                ListItem::new(format!("URI: {}", stream.uri.raw)),
                ListItem::new(format!("Scheme: {}", stream.uri.scheme)),
                ListItem::new(format!("Host: {}", stream.uri.host)),
                ListItem::new(format!("Path: {}", stream.uri.path)),
                ListItem::new(format!("Can Control: {}", stream.properties.can_control)),
                ListItem::new(format!("Can Pause: {}", stream.properties.can_pause)),
                ListItem::new(format!("Can Play: {}", stream.properties.can_play)),
                ListItem::new(format!("Can Seek: {}", stream.properties.can_seek)),
                ListItem::new(format!("Can Go Next: {}", stream.properties.can_go_next)),
                ListItem::new(format!("Can Go Previous: {}", stream.properties.can_go_previous)),
            ];

            let list = List::new(details)
                .block(Block::default()
                    .title(" [ Stream Details ] ")
                    .borders(Borders::ALL)
                    .padding(Padding::new(3, 3, 1, 1)) // Increased left padding from 1 to 3
                    .title_style(Style::default().fg(Color::Magenta)))
                .style(Style::default().fg(Color::White));

            let inner_area = apply_margin(area, margin);
            f.render_widget(list, inner_area);
            return;
        }
    }

    let details = Paragraph::new("Select a stream to see details")
        .block(Block::default()
            .title(" [ Stream Details ] ")
            .borders(Borders::ALL)
            .padding(Padding::new(3, 3, 1, 1)) // Increased left padding from 1 to 3
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}