use ratatui::{
    prelude::*,
    widgets::{Paragraph},
    layout::{Rect, Alignment},
    style::{Style, Color},
};
use crate::ui::AppState;

pub fn draw_header(f: &mut Frame, _app_state: &AppState, area: Rect) {
    let header_text = Paragraph::new(format!("[ snaptui v{} ]", env!("CARGO_PKG_VERSION")))

        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center);

    let help_text = Paragraph::new("Keys: [ Tabs: ←/→ | Focus: Tab | Move: ↑/↓ | Select: Space | Edit: e | Confirm: Enter | Cancel: Esc | Quit: q ]")
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