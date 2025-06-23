use ratatui::{
    prelude::*,
    widgets::{Paragraph},
    layout::{Rect, Alignment},
    style::{Style, Color},
};
use crate::ui::AppState;

pub fn draw_header(f: &mut Frame, _app_state: &AppState, area: Rect) {
    let header_text = Paragraph::new("[ snaptui v0.2.0 ]")
        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center);

    let help_text = Paragraph::new("Navigation: [ Tabs ' ← or → ' | List  ' ↑ or ↓ ' | Details ' Tab or Shift+Tab ' | Quit: ' q ' ]")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(area);

    f.render_widget(header_text, layout[0]);
    f.render_widget(help_text, layout[1]);
}