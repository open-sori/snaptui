use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;
use std::io::Result;

pub enum InputEvent {
    Quit,
    None,
}

pub fn handle_input() -> Result<InputEvent> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(InputEvent::Quit),
                _ => {}
            }
        }
    }
    Ok(InputEvent::None)
}