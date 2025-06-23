use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
    style::{Style, Color},
};
use crate::ui::{AppState, utils::apply_margin};

pub fn draw_group_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let margin = 1;

    let mut items = Vec::new();

    if let Some(data) = &*status_data {
        for (i, group) in data.result.server.groups.iter().enumerate() {
            let content = if i == *selected_index {
                format!("> {}", group.id)
            } else {
                format!("  {}", group.id)
            };

            let item = ListItem::new(content)
                .style(if i == *selected_index {
                    Style::default().fg(Color::Magenta).bold()
                } else {
                    Style::default().fg(Color::White)
                });

            items.push(item);
        }
    } else {
        items.push(ListItem::new("No groups available"));
    }

    let list = List::new(items)
        .block(Block::default()
            .title(" [ Groups List ] ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1))
            .border_style(Style::default().fg(Color::Magenta))
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Magenta).bold());

    let inner_area = apply_margin(area, margin);
    f.render_widget(list, inner_area);
}