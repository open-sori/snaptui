use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use crate::ui::{AppState, utils::apply_margin};

pub fn draw_stream_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let margin = 1;

    let mut items = Vec::new();

    if let Some(data) = &*status_data {
        for (i, stream) in data.result.server.streams.iter().enumerate() {
            let content = if i == *selected_index {
                format!("> {}", stream.id)
            } else {
                format!("  {}", stream.id)
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
        items.push(ListItem::new("No streams available"));
    }

    let list = List::new(items)
        .block(Block::default()
            .title(" [ Streams List ] ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .padding(Padding::new(1, 1, 1, 1))
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Magenta).bold());

    let inner_area = apply_margin(area, margin);
    f.render_widget(list, inner_area);
}