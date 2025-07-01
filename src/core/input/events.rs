use crossterm::event::{self, Event, KeyCode};
use std::io::Result;
use std::sync::MutexGuard;
use std::time::Duration;

use crate::ui::{ClientDetailsFocus, GroupDetailsFocus, PanelFocus, TabSelection};

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    Quit,
    TabChanged(TabSelection),
    Up,
    Down,
    ToggleFocus,
    Edit,
    Confirm,
    Cancel,
    Char(char),
    Backspace,
    None,
}

pub fn handle_input(
    current_tab: &mut MutexGuard<'_, TabSelection>,
    selected_index: &mut MutexGuard<'_, usize>,
    _max_items: usize,
    focused_panel: &mut MutexGuard<'_, PanelFocus>,
    is_editing_client_name: bool,
    is_editing_client_volume: bool,
    is_editing_client_latency: bool,
) -> Result<InputEvent> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            if is_editing_client_name || is_editing_client_volume || is_editing_client_latency {
                match key_event.code {
                    KeyCode::Enter => return Ok(InputEvent::Confirm),
                    KeyCode::Char(c) => return Ok(InputEvent::Char(c)),
                    KeyCode::Backspace => return Ok(InputEvent::Backspace),
                    KeyCode::Esc => return Ok(InputEvent::Cancel),
                    _ => {}
                }
            } else {
                // Handle regular key presses
                match key_event.code {
                    KeyCode::Char('q') => return Ok(InputEvent::Quit),
                    KeyCode::Tab => return Ok(InputEvent::ToggleFocus),
                    KeyCode::Char('e') => {
                        if **focused_panel == PanelFocus::Details {
                            return Ok(InputEvent::Edit);
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
                        return Ok(InputEvent::Up);
                    }
                    KeyCode::Down => {
                        return Ok(InputEvent::Down);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(InputEvent::None)
}

pub fn get_next_group_field(current_field: &GroupDetailsFocus) -> GroupDetailsFocus {
    match current_field {
        GroupDetailsFocus::Name => GroupDetailsFocus::StreamId,
        GroupDetailsFocus::StreamId => GroupDetailsFocus::Muted,
        GroupDetailsFocus::Muted => GroupDetailsFocus::Clients,
        GroupDetailsFocus::Clients => GroupDetailsFocus::Name,
        _ => GroupDetailsFocus::Name,
    }
}

pub fn get_previous_group_field(current_field: &GroupDetailsFocus) -> GroupDetailsFocus {
    match current_field {
        GroupDetailsFocus::Name => GroupDetailsFocus::Clients,
        GroupDetailsFocus::StreamId => GroupDetailsFocus::Name,
        GroupDetailsFocus::Muted => GroupDetailsFocus::StreamId,
        GroupDetailsFocus::Clients => GroupDetailsFocus::Muted,
        _ => GroupDetailsFocus::Name,
    }
}

pub fn get_next_client_field(current_field: &ClientDetailsFocus) -> ClientDetailsFocus {
    match current_field {
        ClientDetailsFocus::Name => ClientDetailsFocus::Volume,
        ClientDetailsFocus::Volume => ClientDetailsFocus::Muted,
        ClientDetailsFocus::Muted => ClientDetailsFocus::Latency,
        ClientDetailsFocus::Latency => ClientDetailsFocus::Name,
        _ => ClientDetailsFocus::Name,
    }
}

pub fn get_previous_client_field(current_field: &ClientDetailsFocus) -> ClientDetailsFocus {
    match current_field {
        ClientDetailsFocus::Name => ClientDetailsFocus::Latency,
        ClientDetailsFocus::Volume => ClientDetailsFocus::Name,
        ClientDetailsFocus::Muted => ClientDetailsFocus::Volume,
        ClientDetailsFocus::Latency => ClientDetailsFocus::Muted,
        _ => ClientDetailsFocus::Name,
    }
}