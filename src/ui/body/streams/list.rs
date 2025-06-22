use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use crate::ui::AppState;
use crate::ui::utils::apply_margin;

pub fn draw_stream_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let margin = 1;

    let mut items = Vec::new();

    if let Some(data) = &*status_data {
        for stream in &data.result.server.streams {
            items.push(ListItem::new(format!("{} - {}", stream.id, stream.status)));
        }
    } else {
        items.push(ListItem::new("No streams available"));
    }

    let list = List::new(items)
        .block(Block::default()
            .title(" [ Streams List ] ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1))) // Add padding
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(list, inner_area);
}