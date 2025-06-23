use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::io::Result;
use std::sync::MutexGuard;
use crate::ui::TabSelection;
use crate::ui::DetailsFocus;

pub enum InputEvent {
    Quit,
    TabChanged(TabSelection),
    Up,
    Down,
    Refresh,
    Select,
    CycleFields,
    None,
}

pub fn handle_input(
    current_tab: &mut MutexGuard<'_, TabSelection>,
    selected_index: &mut MutexGuard<'_, usize>,
    max_items: usize,
    details_focused: &mut MutexGuard<'_, bool>,
    focused_field: &mut MutexGuard<'_, DetailsFocus>,
) -> Result<InputEvent> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(InputEvent::Quit),
                KeyCode::Char('r') => return Ok(InputEvent::Refresh),
                KeyCode::Enter => {
                    if **details_focused {
                        return Ok(InputEvent::CycleFields);
                    } else {
                        return Ok(InputEvent::Select);
                    }
                }
                KeyCode::Tab => {
                    if **details_focused {
                        return Ok(InputEvent::CycleFields);
                    } else {
                        let new_tab = match **current_tab {
                            TabSelection::Groups => TabSelection::Clients,
                            TabSelection::Clients => TabSelection::Streams,
                            TabSelection::Streams => TabSelection::Groups,
                        };
                        **current_tab = new_tab.clone();
                        **selected_index = 0;
                        return Ok(InputEvent::TabChanged(new_tab));
                    }
                }
                KeyCode::Left => {
                    let new_tab = match **current_tab {
                        TabSelection::Groups => TabSelection::Streams,
                        TabSelection::Clients => TabSelection::Groups,
                        TabSelection::Streams => TabSelection::Clients,
                    };
                    **current_tab = new_tab.clone();
                    **selected_index = 0;
                    return Ok(InputEvent::TabChanged(new_tab));
                }
                KeyCode::Right => {
                    let new_tab = match **current_tab {
                        TabSelection::Groups => TabSelection::Clients,
                        TabSelection::Clients => TabSelection::Streams,
                        TabSelection::Streams => TabSelection::Groups,
                    };
                    **current_tab = new_tab.clone();
                    **selected_index = 0;
                    return Ok(InputEvent::TabChanged(new_tab));
                }
                KeyCode::Up => {
                    if **selected_index > 0 {
                        **selected_index -= 1;
                        return Ok(InputEvent::Up);
                    }
                }
                KeyCode::Down => {
                    if **selected_index < max_items.saturating_sub(1) {
                        **selected_index += 1;
                        return Ok(InputEvent::Down);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(InputEvent::None)
}