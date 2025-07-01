use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
    style::{Style, Color},
};
use crate::ui::{AppState, utils::apply_margin};
use super::GroupDetailsFocus;

pub fn draw_group_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let group_focused_field = app_state.group_focused_field.lock().unwrap();
    let margin = 1;

    if let Some(data) = &*status_data {
        if data.result.server.groups.len() > *selected_index {
            let group = &data.result.server.groups[*selected_index];

            let mut details = Vec::new();

            // Add group ID field
            details.push(ListItem::new(format!("  Id: {}", group.id))
                .style(Style::default().fg(Color::White)));

            // Add name field with potential highlighting
            let name_text = if *group_focused_field == GroupDetailsFocus::Name {
                format!("> Name: {}", group.name)
            } else {
                format!("  Name: {}", group.name)
            };
            let name_style = if *group_focused_field == GroupDetailsFocus::Name {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(name_text).style(name_style));

            // Add stream ID field with potential highlighting
            let stream_id_text = if *group_focused_field == GroupDetailsFocus::StreamId {
                format!("> Stream Id: {}", group.stream_id)
            } else {
                format!("  Stream Id: {}", group.stream_id)
            };
            let stream_id_style = if *group_focused_field == GroupDetailsFocus::StreamId {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(stream_id_text).style(stream_id_style));

            // Add muted field with potential highlighting
            let muted_text = if *group_focused_field == GroupDetailsFocus::Muted {
                format!("> Muted: {:>5}", group.muted)
            } else {
                format!("  Muted: {:>5}", group.muted)
            };
            let muted_style = if *group_focused_field == GroupDetailsFocus::Muted {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(muted_text).style(muted_style));

            // Add clients section header with potential highlighting
            let clients_text = if *group_focused_field == GroupDetailsFocus::Clients {
                "> Clients:".to_string()
            } else {
                "  Clients:".to_string()
            };
            let clients_style = if *group_focused_field == GroupDetailsFocus::Clients {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::White)
            };
            details.push(ListItem::new(clients_text).style(clients_style));

            // Add each client as a sub-item
            for client in &group.clients {
                details.push(ListItem::new(format!("  - Id: {}", client.id)));
                details.push(ListItem::new(format!("    Connected: {}", client.connected)));
            }

            let list = List::new(details)
                .block(Block::default()
                    .title(" [ Group Details ] ")
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

    let details = Paragraph::new("Select a group to see details")
        .block(Block::default()
            .title(" [ Group Details ] ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(3, 3, 1, 1))
            .title_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}