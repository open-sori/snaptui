use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::io::Result;

pub enum InputEvent {
    Quit,
    None,
}

pub fn handle_input() -> Result<InputEvent> {
    if event::poll(Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                return Ok(InputEvent::Quit);
            }
        }
    }
    Ok(InputEvent::None)
}