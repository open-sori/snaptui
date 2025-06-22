use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;
use std::io::Result;
use crate::ui::TabSelection;

pub enum InputEvent {
    Quit,
    SwitchTab(usize),
    None,
}

pub fn handle_input(current_tab: &mut TabSelection) -> Result<InputEvent> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(InputEvent::Quit),
                KeyCode::Tab => {
                    // Cycle through tabs
                    *current_tab = match current_tab {
                        TabSelection::Groups => TabSelection::Clients,
                        TabSelection::Clients => TabSelection::Streams,
                        TabSelection::Streams => TabSelection::Groups,
                    };
                    return Ok(InputEvent::None);
                }
                KeyCode::Left => {
                    // Switch to the previous tab
                    *current_tab = match current_tab {
                        TabSelection::Groups => TabSelection::Streams,
                        TabSelection::Clients => TabSelection::Groups,
                        TabSelection::Streams => TabSelection::Clients,
                    };
                    return Ok(InputEvent::None);
                }
                KeyCode::Right => {
                    // Switch to the next tab
                    *current_tab = match current_tab {
                        TabSelection::Groups => TabSelection::Clients,
                        TabSelection::Clients => TabSelection::Streams,
                        TabSelection::Streams => TabSelection::Groups,
                    };
                    return Ok(InputEvent::None);
                }
                _ => {}
            }
        }
    }
    Ok(InputEvent::None)
}