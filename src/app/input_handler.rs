use std::sync::MutexGuard;
use std::io::Result;
use crate::ui::{TabSelection, GroupDetailsFocus, ClientDetailsFocus};
use crate::input::{handle_input, InputEvent};
use crate::models::server::getstatus::GetStatusData;

pub fn handle_app_input(
    current_tab: &mut MutexGuard<'_, TabSelection>,
    selected_index: &mut MutexGuard<'_, usize>,
    details_focused: &mut MutexGuard<'_, bool>,
    group_focused_field: &mut MutexGuard<'_, GroupDetailsFocus>,
    client_focused_field: &mut MutexGuard<'_, ClientDetailsFocus>,
    status_data: &MutexGuard<'_, Option<GetStatusData>>,
) -> Result<Option<InputEvent>> {
    let max_items = if let Some(data) = &**status_data {
        match **current_tab {
            TabSelection::Groups => data.result.server.groups.len(),
            TabSelection::Clients => {
                data.result.server.groups.iter()
                    .map(|g| g.clients.len())
                    .sum()
            },
            TabSelection::Streams => data.result.server.streams.len(),
        }
    } else {
        0
    };

    match handle_input(
        current_tab,
        selected_index,
        max_items,
        details_focused,
        group_focused_field,
        client_focused_field,
    ) {
        Ok(event) => {
            match event {
                InputEvent::Quit => return Ok(Some(InputEvent::Quit)),
                InputEvent::TabChanged(new_tab) => {
                    **current_tab = new_tab;
                    **selected_index = 0;
                    **details_focused = false;
                    **group_focused_field = GroupDetailsFocus::None;
                    **client_focused_field = ClientDetailsFocus::None;
                },
                InputEvent::Up | InputEvent::Down => {
                    if **details_focused {
                        match **current_tab {
                            TabSelection::Groups => {
                                **group_focused_field = GroupDetailsFocus::Name;
                            },
                            TabSelection::Clients => {
                                **client_focused_field = ClientDetailsFocus::Name;
                            },
                            _ => {}
                        }
                    }
                },
                InputEvent::CycleFields | InputEvent::None => {}
            }
            Ok(None)
        },
        Err(e) => {
            log::error!("Error handling input: {}", e);
            Err(e)
        }
    }
}

pub fn check_auto_focus(
    details_focused: &mut MutexGuard<'_, bool>,
    current_tab: &MutexGuard<'_, TabSelection>,
    group_focused_field: &mut MutexGuard<'_, GroupDetailsFocus>,
    client_focused_field: &mut MutexGuard<'_, ClientDetailsFocus>,
    status_data: &MutexGuard<'_, Option<GetStatusData>>,
) {
    if !**details_focused {
        match **current_tab {
            TabSelection::Groups => {
                if let Some(data) = &**status_data {
                    if !data.result.server.groups.is_empty() {
                        **details_focused = true;
                        **group_focused_field = GroupDetailsFocus::Name;
                    }
                }
            },
            TabSelection::Clients => {
                if let Some(data) = &**status_data {
                    let client_count: usize = data.result.server.groups.iter()
                        .map(|g| g.clients.len())
                        .sum();
                    if client_count > 0 {
                        **details_focused = true;
                        **client_focused_field = ClientDetailsFocus::Name;
                    }
                }
            },
            _ => {}
        }
    }
}