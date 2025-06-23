use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem, Padding},
};
use crate::ui::{AppState, utils::apply_margin};

pub fn draw_group_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let margin = 1;

    if let Some(data) = &*status_data {
        if data.result.server.groups.len() > *selected_index {
            let group = &data.result.server.groups[*selected_index];

            // Create a list of details including clients information
            let mut details = vec![
                ListItem::new(format!("Id: {}", group.id)),
                ListItem::new(format!("Name: {}", group.name)),
                ListItem::new(format!("Stream Id: {}", group.stream_id)),
                ListItem::new(format!("Muted: {}", group.muted)),
                ListItem::new("Clients:".to_string()),
            ];

            // Add each client as a sub-item
            for client in &group.clients {
                details.push(ListItem::new(format!("- Id: {}", client.id)));
                details.push(ListItem::new(format!("  Connected: {}", client.connected)));
            }

            let list = List::new(details)
                .block(Block::default()
                    .title(" [ Group Details ] ")
                    .borders(Borders::ALL)
                    .padding(Padding::new(3, 3, 1, 1))
                    .title_style(Style::default().fg(Color::Magenta)))
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
            .padding(Padding::new(3, 3, 1, 1))
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}