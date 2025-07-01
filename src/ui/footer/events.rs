use crate::ui::utils::apply_margin;
use crate::ui::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Padding},
    Frame,
};

pub fn draw_events(f: &mut Frame, app_state: &AppState, area: Rect) {
    let message = app_state.last_message.lock().unwrap();
    let margin = 1;

    let display_message = if message.is_empty() {
        "Notifications will appear here.".to_string()
    } else {
        message.clone()
    };

    let items: Vec<ListItem> =
        vec![ListItem::new(display_message).style(Style::default().fg(Color::White))];

    let events_block = List::new(items)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(" [ Events ] ")
                .padding(Padding::new(2, 2, 1, 1))
                .title_style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(events_block, inner_area);
}