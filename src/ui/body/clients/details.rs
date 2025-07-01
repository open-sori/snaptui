use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
    style::{Style, Color},
};
use crate::ui::{AppState, utils::apply_margin, PanelFocus};
use super::ClientDetailsFocus;

pub fn draw_client_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let client_focused_field = app_state.client_focused_field.lock().unwrap();
    let focused_panel = app_state.focused_panel.lock().unwrap();
    let is_editing_name = *app_state.is_editing_client_name.lock().unwrap();
    let is_editing_volume = *app_state.is_editing_client_volume.lock().unwrap();
    let is_editing_latency = *app_state.is_editing_client_latency.lock().unwrap();
    let margin = 1;

    let is_details_focused = *focused_panel == PanelFocus::Details;

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

                    let name_text = if *client_focused_field == ClientDetailsFocus::Name && is_details_focused {
                        if is_editing_name {
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

                    let name_style = if *client_focused_field == ClientDetailsFocus::Name && is_details_focused {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(name_text).style(name_style));

                    // Add volume field with potential highlighting and editing
                    let volume_text = if *client_focused_field == ClientDetailsFocus::Volume && is_details_focused {
                        if is_editing_volume {
                            let editing_volume = app_state.editing_client_volume.lock().unwrap();
                            let cursor_visible = app_state.cursor_visible.lock().unwrap();
                            let cursor = if *cursor_visible { "_" } else { " " };
                            format!("> Volume: {}{}%", *editing_volume, cursor)
                        } else {
                            format!("> Volume: {}%", client.config.volume.percent)
                        }
                    } else {
                        format!("  Volume: {}%", client.config.volume.percent)
                    };
                    let volume_style = if *client_focused_field == ClientDetailsFocus::Volume && is_details_focused {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(volume_text).style(volume_style));

                    // Add muted field with potential highlighting
                    let muted_text = if *client_focused_field == ClientDetailsFocus::Muted && is_details_focused {
                        format!("> Muted: {}", client.config.volume.muted)
                    } else {
                        format!("  Muted: {}", client.config.volume.muted)
                    };
                    let muted_style = if *client_focused_field == ClientDetailsFocus::Muted && is_details_focused {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(muted_text).style(muted_style));

                    // Add latency field with potential highlighting
                    let latency_text = if *client_focused_field == ClientDetailsFocus::Latency && is_details_focused {
                        if is_editing_latency {
                            let editing_latency = app_state.editing_client_latency.lock().unwrap();
                            let cursor_visible = app_state.cursor_visible.lock().unwrap();
                            let cursor = if *cursor_visible { "_" } else { " " };
                            format!("> Latency: {}{} ms", *editing_latency, cursor)
                        } else {
                            format!("> Latency: {} ms", client.config.latency)
                        }
                    } else {
                        format!("  Latency: {} ms", client.config.latency)
                    };
                    let latency_style = if *client_focused_field == ClientDetailsFocus::Latency && is_details_focused {
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