use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use crate::ui::AppState;
use crate::ui::utils::apply_margin;

pub fn draw_group_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let margin = 1;

    let mut items = Vec::new();

    if let Some(data) = &*status_data {
        for group in &data.result.server.groups {
            items.push(ListItem::new(format!("{} - {}", group.id, group.name)));
        }
    } else {
        items.push(ListItem::new("No groups available"));
    }

    let list = List::new(items)
        .block(Block::default()
            .title(" [ Groups List ] ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(list, inner_area);
}