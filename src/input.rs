use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::io::Result;
use crate::ui::TabSelection;

pub enum InputEvent {
    Quit,
    TabChanged,
    Up,
    Down,
    Refresh,  // Add a new variant for refresh
    None,
}

pub fn handle_input(
    current_tab: &mut TabSelection,
    selected_index: &mut usize,
    max_items: usize,
) -> Result<InputEvent> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(InputEvent::Quit),
                KeyCode::Char('r') => return Ok(InputEvent::Refresh),  // Handle 'r' key for refresh
                KeyCode::Tab => {
                    *current_tab = match current_tab {
                        TabSelection::Groups => TabSelection::Clients,
                        TabSelection::Clients => TabSelection::Streams,
                        TabSelection::Streams => TabSelection::Groups,
                    };
                    *selected_index = 0; // Reset selection when changing tabs
                    return Ok(InputEvent::TabChanged);
                }
                KeyCode::Left => {
                    *current_tab = match current_tab {
                        TabSelection::Groups => TabSelection::Streams,
                        TabSelection::Clients => TabSelection::Groups,
                        TabSelection::Streams => TabSelection::Clients,
                    };
                    *selected_index = 0; // Reset selection when changing tabs
                    return Ok(InputEvent::TabChanged);
                }
                KeyCode::Right => {
                    *current_tab = match current_tab {
                        TabSelection::Groups => TabSelection::Clients,
                        TabSelection::Clients => TabSelection::Streams,
                        TabSelection::Streams => TabSelection::Groups,
                    };
                    *selected_index = 0; // Reset selection when changing tabs
                    return Ok(InputEvent::TabChanged);
                }
                KeyCode::Up => {
                    if *selected_index > 0 {
                        *selected_index -= 1;
                        return Ok(InputEvent::Up);
                    }
                }
                KeyCode::Down => {
                    if *selected_index < max_items.saturating_sub(1) {
                        *selected_index += 1;
                        return Ok(InputEvent::Down);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(InputEvent::None)
}