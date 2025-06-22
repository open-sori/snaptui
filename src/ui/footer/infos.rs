use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Rect, Alignment},
    style::{Style, Color},
};
use crate::ui::AppState;

pub fn draw_infos(f: &mut Frame, app_state: &AppState, area: Rect) {
    let message = app_state.last_message.lock().unwrap();

    let info_block = Paragraph::new(message.as_str())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title(" [ Events ] "));

    f.render_widget(info_block, area);
}