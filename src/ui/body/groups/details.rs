use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Padding},
};
use crate::ui::AppState;
use crate::ui::utils::apply_margin;

pub fn draw_group_details(f: &mut Frame, app_state: &AppState, area: Rect) {
    let status_data = app_state.status_data.lock().unwrap();
    let margin = 1;

    let content = if let Some(data) = &*status_data {
        if let Some(group) = data.result.server.groups.first() {
            format!(
                "ID: {}\nName: {}\nStream ID: {}\nMuted: {}",
                group.id, group.name, group.stream_id, group.muted
            )
        } else {
            "Select a group to see details".to_string()
        }
    } else {
        "No group data available".to_string()
    };

    let details = Paragraph::new(content)
        .block(Block::default()
            .title(" [ Group Details ] ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1)))
        .style(Style::default().fg(Color::White));

    let inner_area = apply_margin(area, margin);
    f.render_widget(details, inner_area);
}