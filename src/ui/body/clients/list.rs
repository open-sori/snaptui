use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use crate::ui::{AppState, utils::apply_margin};

pub fn draw_client_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let margin = 1;

    let mut items = Vec::new();
    let mut current_index = 0;

    if let Some(data) = &*status_data {
        for group in &data.result.server.groups {
            for client in &group.clients {
                let content = if current_index == *selected_index {
                    format!("> {}", client.id)
                } else {
                    format!("  {}", client.id)
                };

                let item = ListItem::new(content)
                    .style(if current_index == *selected_index {
                        Style::default().fg(Color::Magenta).bold()
                    } else {
                        Style::default().fg(Color::White)
                    });

                items.push(item);
                current_index += 1;
            }
        }
    } else {
        items.push(ListItem::new("No clients available"));
    }

    let list = List::new(items)
        .block(Block::default()
            .title(" [ Clients List ] ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .padding(Padding::new(1, 1, 1, 1))
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Magenta).bold());

    let inner_area = apply_margin(area, margin);
    f.render_widget(list, inner_area);
}