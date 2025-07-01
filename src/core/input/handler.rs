use std::sync::MutexGuard;
use crate::ui::{TabSelection, GroupDetailsFocus, ClientDetailsFocus};
use crate::models::server::getstatus::GetStatusData;

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