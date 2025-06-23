use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
    style::{Style, Color},
};
use crate::ui::{AppState, utils::apply_margin, DetailsFocus};

pub fn draw_client_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let details_focused = app_state.details_focused.lock().unwrap();
    let focused_field = app_state.focused_field.lock().unwrap();
    let margin = 1;

    // Determine border color based on focus state
    let border_color = if *details_focused {
        Color::Yellow
    } else {
        Color::White
    };

    if let Some(data) = &*status_data {
        let mut client_count = 0;
        for group in &data.result.server.groups {
            for client in &group.clients {
                if client_count == *selected_index {
                    let mut details = Vec::new();

                    // Add other fields
                    details.push(ListItem::new(format!("  Id: {}", client.id)));
                    details.push(ListItem::new(format!("  Instance: {}", client.config.instance)));
                    details.push(ListItem::new(format!("  Connected: {}", client.connected)));
                    details.push(ListItem::new(format!("  Version: {}", client.snapclient.version)));
                    details.push(ListItem::new(format!("  Ip: {}", client.host.ip)));
                    details.push(ListItem::new(format!("  Mac: {}", client.host.mac)));
                    details.push(ListItem::new(format!("  Name: {}", client.config.name)));

                    // Add volume field with potential highlighting
                    let volume_text = if *focused_field == DetailsFocus::Volume {
                        format!("> Volume: {:>3}%", client.config.volume.percent)
                    } else {
                        format!("  Volume: {:>3}%", client.config.volume.percent)
                    };
                    let volume_style = if *focused_field == DetailsFocus::Volume {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(volume_text).style(volume_style));

                    // Add muted field with potential highlighting
                    let muted_text = if *focused_field == DetailsFocus::Muted {
                        format!("> Muted: {:>5}", client.config.volume.muted)
                    } else {
                        format!("  Muted: {:>5}", client.config.volume.muted)
                    };
                    let muted_style = if *focused_field == DetailsFocus::Muted {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    details.push(ListItem::new(muted_text).style(muted_style));

                    // Add latency field with potential highlighting
                    let latency_text = if *focused_field == DetailsFocus::Latency {
                        format!("> Latency: {}", client.config.latency)
                    } else {
                        format!("  Latency: {}", client.config.latency)
                    };
                    let latency_style = if *focused_field == DetailsFocus::Latency {
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