use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
};
use crate::ui::{AppState, utils::apply_margin};

pub fn draw_client_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let margin = 1;

    if let Some(data) = &*status_data {
        let mut client_count = 0;
        for group in &data.result.server.groups {
            for client in &group.clients {
                if client_count == *selected_index {
                    let details = vec![
                        ListItem::new(format!("ID: {}", client.id)),
                        ListItem::new(format!("Name: {}", client.config.name)),
                        ListItem::new(format!("Connected: {}", client.connected)),
                        ListItem::new(format!("Volume: {}%", client.config.volume.percent)),
                        ListItem::new(format!("Muted: {}", client.config.volume.muted)),
                        ListItem::new(format!("Instance: {}", client.config.instance)),
                        ListItem::new(format!("Latency: {}", client.config.latency)),
                        ListItem::new(format!("Host: {}", client.host.name)),
                        ListItem::new(format!("IP: {}", client.host.ip)),
                        ListItem::new(format!("OS: {}", client.host.os)),
                        ListItem::new(format!("Snapclient Version: {}", client.snapclient.version)),
                    ];

                    let list = List::new(details)
                        .block(Block::default()
                            .title(" [ Client Details ] ")
                            .borders(Borders::ALL)
                            .padding(Padding::new(3, 3, 1, 1)) // Increased left padding from 1 to 3
                            .title_style(Style::default().fg(Color::Magenta)))
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
            .padding(Padding::new(3, 3, 1, 1)) // Increased left padding from 1 to 3
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}