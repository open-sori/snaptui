use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::io::Result;
use std::sync::MutexGuard;
use crate::ui::TabSelection;
use crate::ui::GroupDetailsFocus;
use crate::ui::ClientDetailsFocus;

pub enum InputEvent {
    Quit,
    TabChanged(TabSelection),
    Up,
    Down,
    Refresh,
    CycleFields,
    None,
}

pub fn handle_input(
    current_tab: &mut MutexGuard<'_, TabSelection>,
    selected_index: &mut MutexGuard<'_, usize>,
    max_items: usize,
    details_focused: &mut MutexGuard<'_, bool>,
    group_focused_field: &mut MutexGuard<'_, GroupDetailsFocus>,
    client_focused_field: &mut MutexGuard<'_, ClientDetailsFocus>,
) -> Result<InputEvent> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(InputEvent::Quit),
                KeyCode::Char('r') => return Ok(InputEvent::Refresh),
                KeyCode::BackTab => {
                    if **details_focused {
                        match **current_tab {
                            TabSelection::Groups => {
                                let current_field = group_focused_field.clone();
                                **group_focused_field = get_previous_group_field(&current_field);
                            },
                            TabSelection::Clients => {
                                let current_field = client_focused_field.clone();
                                **client_focused_field = get_previous_client_field(&current_field);
                            },
                            _ => {}
                        }
                        return Ok(InputEvent::CycleFields);
                    }
                }
                KeyCode::Tab => {
                    if **details_focused {
                        match **current_tab {
                            TabSelection::Groups => {
                                let current_field = group_focused_field.clone();
                                **group_focused_field = get_next_group_field(&current_field);
                            },
                            TabSelection::Clients => {
                                let current_field = client_focused_field.clone();
                                **client_focused_field = get_next_client_field(&current_field);
                            },
                            _ => {}
                        }
                        return Ok(InputEvent::CycleFields);
                    } else {
                        // If not in details focus, use Tab to switch focus to details
                        **details_focused = true;
                        match **current_tab {
                            TabSelection::Groups => {
                                **group_focused_field = GroupDetailsFocus::Name;
                            },
                            TabSelection::Clients => {
                                **client_focused_field = ClientDetailsFocus::Name;
                            },
                            _ => {}
                        }
                        return Ok(InputEvent::CycleFields);
                    }
                }
                KeyCode::Left => {
                    // Navigate left in tabs
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
                    // Navigate right in tabs
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
                    // Navigate up in list
                    if **selected_index > 0 {
                        **selected_index -= 1;
                        return Ok(InputEvent::Up);
                    }
                }
                KeyCode::Down => {
                    // Navigate down in list
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

fn get_next_group_field(current_field: &GroupDetailsFocus) -> GroupDetailsFocus {
    match current_field {
        GroupDetailsFocus::Name => GroupDetailsFocus::StreamId,
        GroupDetailsFocus::StreamId => GroupDetailsFocus::Muted,
        GroupDetailsFocus::Muted => GroupDetailsFocus::Clients,
        GroupDetailsFocus::Clients => GroupDetailsFocus::Name,
        _ => GroupDetailsFocus::Name,
    }
}

fn get_previous_group_field(current_field: &GroupDetailsFocus) -> GroupDetailsFocus {
    match current_field {
        GroupDetailsFocus::Name => GroupDetailsFocus::Clients,
        GroupDetailsFocus::StreamId => GroupDetailsFocus::Name,
        GroupDetailsFocus::Muted => GroupDetailsFocus::StreamId,
        GroupDetailsFocus::Clients => GroupDetailsFocus::Muted,
        _ => GroupDetailsFocus::Name,
    }
}

fn get_next_client_field(current_field: &ClientDetailsFocus) -> ClientDetailsFocus {
    match current_field {
        ClientDetailsFocus::Name => ClientDetailsFocus::Volume,
        ClientDetailsFocus::Volume => ClientDetailsFocus::Latency,
        ClientDetailsFocus::Latency => ClientDetailsFocus::Name,
        _ => ClientDetailsFocus::Name,
    }
}

fn get_previous_client_field(current_field: &ClientDetailsFocus) -> ClientDetailsFocus {
    match current_field {
        ClientDetailsFocus::Name => ClientDetailsFocus::Latency,
        ClientDetailsFocus::Volume => ClientDetailsFocus::Name,
        ClientDetailsFocus::Latency => ClientDetailsFocus::Volume,
        _ => ClientDetailsFocus::Name,
    }
}