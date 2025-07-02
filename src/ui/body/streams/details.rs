use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
    style::{Style, Color},
};
use crate::ui::{AppState, utils::apply_margin, PanelFocus};

pub fn draw_stream_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let focused_panel = app_state.focused_panel.lock().unwrap();
    let margin = 1;

    let is_details_focused = *focused_panel == PanelFocus::Details;
    let title = if is_details_focused { " [ * Stream Details * ] " } else { " [ Stream Details ] " };

    if let Some(data) = &*status_data {
        if data.result.server.streams.len() > *selected_index {
            let stream = &data.result.server.streams[*selected_index];
            let mut details = Vec::new();

            details.push(ListItem::new(format!("  Id: {}", stream.id))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Status: {}", stream.status))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Host: {}", stream.uri.host))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Name: {}", stream.uri.query.name))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Scheme: {}", stream.uri.scheme))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Path: {}", stream.uri.path))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Chunk Ms: {}", stream.uri.query.chunk_ms))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Codec: {}", stream.uri.query.codec))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Sample Format: {}", stream.uri.query.sampleformat))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Can Control: {}", stream.properties.can_control))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Can Pause: {}", stream.properties.can_pause))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Can Play: {}", stream.properties.can_play))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Can Seek: {}", stream.properties.can_seek))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Can Go Next: {}", stream.properties.can_go_next))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Can Go Previous: {}", stream.properties.can_go_previous))
                .style(Style::default().fg(Color::White)));

            details.push(ListItem::new(format!("  Uri: {}", stream.uri.raw))
                .style(Style::default().fg(Color::White)));

            let list = List::new(details)
                .block(Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .padding(Padding::new(3, 3, 1, 1))
                    .title_style(Style::default().fg(Color::Yellow)))
                .style(Style::default().fg(Color::White));

            let inner_area = apply_margin(area, margin);
            f.render_widget(list, inner_area);
            return;
        }
    }

    let details = Paragraph::new("Select a stream to see details")
        .block(Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(3, 3, 1, 1))
            .title_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}