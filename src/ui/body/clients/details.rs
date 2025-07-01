use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
    style::{Style, Color},
};
use crate::ui::{AppState, utils::apply_margin};
use super::ClientDetailsFocus;

pub fn draw_client_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let client_focused_field = app_state.client_focused_field.lock().unwrap();
    let is_editing = *app_state.is_editing_client_name.lock().unwrap();
    let margin = 1;

    if let Some(data) = &*status_data {
        let mut client_count = 0;
        for group in &data.result.server.groups {
            for client in &group.clients {
                if client_count == *selected_index {
                    let mut details = Vec::new();
                    details.push(ListItem::new(format!("  Id: {}", client.id)));
                    details.push(ListItem::new(format!("  Connected: {}", client.connected)));
                    details.push(ListItem::new(format!("  Version: {}", client.snapclient.version)));
                    details.push(ListItem::new(format!("  Ip: {}", client.host.ip)));
                    details.push(ListItem::new(format!("  Mac: {}", client.host.mac)));

                    let name_text = if *client_focused_field == ClientDetailsFocus::Name {
                        if is_editing {
                            let editing_name = app_state.editing_client_name.lock().unwrap();
                            let cursor_visible = app_state.cursor_visible.lock().unwrap();
                            let cursor = if *cursor_visible { "_" } else { " " };
                            format!("> Name: {}{}", *editing_name, cursor)
                        } else {
                            format!("> Name: {}", client.config.name)
                        }
                    } else {
                        format!("  Name: {}", client.config.name)
                    };

                    let name_style = if *client_focused_field == ClientDetailsFocus::Name {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(name_text).style(name_style));

                    // Add volume field with potential highlighting
                    let volume_text = if *client_focused_field == ClientDetailsFocus::Volume {
                        format!("> Volume: {:>3}%", client.config.volume.percent)
                    } else {
                        format!("  Volume: {:>3}%", client.config.volume.percent)
                    };
                    let volume_style = if *client_focused_field == ClientDetailsFocus::Volume {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(volume_text).style(volume_style));

                    // Add latency field with potential highlighting
                    let latency_text = if *client_focused_field == ClientDetailsFocus::Latency {
                        format!("> Latency: {}", client.config.latency)
                    } else {
                        format!("  Latency: {}", client.config.latency)
                    };
                    let latency_style = if *client_focused_field == ClientDetailsFocus::Latency {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(latency_text).style(latency_style));

                    let list = List::new(details)
                        .block(Block::default()
                            .title(" [ Client Details ] ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(3, 3, 1, 1))
            .title_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White));

                    let inner_area = apply_margin(area, margin);
                    f.render_widget(list, inner_area);
                    return;
                }
                client_count += 1;
            }
        }
    }

    let details = Paragraph::new("Select a client to see details")
        .block(Block::default()
            .title(" [ Client Details ] ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(3, 3, 1, 1))
            .title_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}