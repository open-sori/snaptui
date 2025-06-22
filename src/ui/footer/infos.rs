use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
    layout::{Rect},
    style::{Style, Color},
};
use crate::ui::AppState;
use crate::ui::utils::apply_margin;

pub fn draw_infos(f: &mut Frame, app_state: &AppState, area: Rect) {
    let message = app_state.last_message.lock().unwrap();
    let margin = 1;

    // Split the message into lines
    let lines: Vec<&str> = message.lines().collect();

    // Create a list of recent messages (up to 3)
    let items: Vec<ListItem> = lines.iter()
        .rev() // Show newest first
        .take(3)
        .enumerate()
        .map(|(i, line)| {
            let color = match i {
                0 => Color::Green,  // Most recent
                1 => Color::Yellow,
                _ => Color::Gray,
            };
            ListItem::new(line.to_string()).style(Style::default().fg(color))
        })
        .collect();

    let info_block = List::new(items)
        .style(Style::default().fg(Color::White))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" [ Events ] ")
            .padding(Padding::new(2, 2, 1, 1))
            .title_style(Style::default().fg(Color::Magenta)))
        .highlight_style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(info_block, inner_area);
}