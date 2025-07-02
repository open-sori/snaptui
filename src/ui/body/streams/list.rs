use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
};
use crate::ui::{AppState, utils::apply_margin, PanelFocus};

pub fn draw_stream_list(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let selected_index = app_state.selected_index.lock().unwrap();
    let focused_panel = app_state.focused_panel.lock().unwrap();
    let margin = 1;

    let is_focused = *focused_panel == PanelFocus::List;
    let title = if is_focused { " [ * Streams List * ] " } else { " [ Streams List ] " };

    let mut items = Vec::new();

    if let Some(data) = &*status_data {
        for (i, stream) in data.result.server.streams.iter().enumerate() {
            let is_selected = i == *selected_index;

            let content = if is_selected && is_focused {
                format!("> {}", stream.id)
            } else {
                format!("  {}", stream.id)
            };

            let item = ListItem::new(content)
                .style(if is_selected {
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
            .title(title)
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1))
            .border_style(Style::default().fg(Color::Magenta))
            .title_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Magenta).bold());

    let inner_area = apply_margin(area, margin);
    f.render_widget(list, inner_area);
}