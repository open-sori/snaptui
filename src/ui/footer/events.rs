use crate::ui::utils::apply_margin;
use crate::ui::{AppState, PanelFocus};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

pub fn draw_events(f: &mut Frame, app_state: &AppState, area: Rect) {
    let events = app_state.events.lock().unwrap();
    let focused_panel = app_state.focused_panel.lock().unwrap();
    let margin = 1;

    let is_focused = *focused_panel == PanelFocus::Events;
    let title = if is_focused { " [ * Events * ] " } else { " [ Events ] " };

    let event_text = events
        .iter()
        .rev()
        .take(5)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("\n\n");

    let events_paragraph = Paragraph::new(event_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(title)
                .padding(Padding::new(2, 2, 1, 1))
                .title_style(Style::default().fg(Color::White)),
        );

    let inner_area = apply_margin(area, margin);
    f.render_widget(events_paragraph, inner_area);
}