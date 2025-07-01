use crate::commands::server::getstatus::{create_status_request, extract_server_version};
use crate::core::input::events::handle_input;
use crate::core::input::{check_auto_focus, InputEvent};
use crate::core::websocket::ConnectionStatus;
use crate::models::server::getstatus::GetStatusData;
use crate::ui::{
    initialize_terminal, restore_terminal, AppState, ClientDetailsFocus, GroupDetailsFocus,
    TabSelection,
};
use crate::ui::{draw_ui};
use serde_json::Value;
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

pub struct Application {
    pub terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    pub app_state: AppState,
    pub status_data: Arc<Mutex<Option<GetStatusData>>>,
    pub cmd_tx: mpsc::Sender<String>,
}

impl Application {
    pub fn new(cmd_tx: mpsc::Sender<String>) -> Result<Self> {
        let terminal = initialize_terminal()?;
        let status_data = Arc::new(Mutex::new(None));
        let app_state = AppState {
            last_message: Arc::new(Mutex::new(String::from(""))),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            server_version: Arc::new(Mutex::new(String::new())),
            status_data: Arc::clone(&status_data),
            active_tab: Arc::new(Mutex::new(TabSelection::Groups)),
            selected_index: Arc::new(Mutex::new(0)),
            details_focused: Arc::new(Mutex::new(false)),
            group_focused_field: Arc::new(Mutex::new(GroupDetailsFocus::None)),
            client_focused_field: Arc::new(Mutex::new(ClientDetailsFocus::None)),
            is_editing_client_name: Arc::new(Mutex::new(false)),
            editing_client_name: Arc::new(Mutex::new(String::new())),
            cursor_visible: Arc::new(Mutex::new(true)),
            last_cursor_toggle: Arc::new(Mutex::new(Instant::now())),
        };

        Ok(Self {
            terminal,
            app_state,
            status_data,
            cmd_tx,
        })
    }

    pub async fn run(
        &mut self,
        mut message_rx: mpsc::Receiver<String>,
        mut status_rx: mpsc::Receiver<ConnectionStatus>,
    ) -> Result<()> {
        let status_arc: Arc<Mutex<ConnectionStatus>> = Arc::clone(&self.app_state.status);
        tokio::spawn(async move {
            while let Some(new_status) = status_rx.recv().await {
                if let Ok(mut status) = status_arc.lock() {
                    *status = new_status;
                }
            }
        });

        let status_data_arc: Arc<Mutex<Option<GetStatusData>>> = Arc::clone(&self.status_data);
        let message_arc = Arc::clone(&self.app_state.last_message);
        let version_arc = Arc::clone(&self.app_state.server_version);
        let cmd_tx_clone = self.cmd_tx.clone();

        tokio::spawn(async move {
            while let Some(msg) = message_rx.recv().await {
                log::debug!("Received WebSocket message: {}", msg);
                if let Ok(json_value) = serde_json::from_str::<Value>(&msg) {
                    if let Some(result) = json_value.get("result") {
                        if result.get("server").is_some() {
                            if let Ok(status) =
                                crate::commands::server::getstatus::parse_status_response(&msg)
                            {
                                if let Ok(mut data) = status_data_arc.lock() {
                                    *data = Some(status.clone());
                                }
                                if let Some(version) = extract_server_version(&msg) {
                                    if let Ok(mut version_lock) = version_arc.lock() {
                                        *version_lock = version;
                                    }
                                }
                            }
                        } else {
                            let status_request = create_status_request();
                            if let Err(e) = cmd_tx_clone.send(status_request).await {
                                log::error!(
                                    "Failed to send status request after command: {}",
                                    e
                                );
                            }
                        }
                    } else if json_value.get("method").is_some() {
                        log::debug!("Identified as notification, updating last_message.");
                        if let Ok(mut message) = message_arc.lock() {
                            *message = msg.clone();
                        }
                    }
                }
            }
        });

        loop {
            // Handle cursor blinking
            {
                let mut last_toggle = self.app_state.last_cursor_toggle.lock().unwrap();
                if last_toggle.elapsed() > Duration::from_millis(500) {
                    let mut visible = self.app_state.cursor_visible.lock().unwrap();
                    *visible = !*visible;
                    *last_toggle = Instant::now();
                }
            }

            if let Err(e) = draw_ui(&mut self.terminal, &self.app_state) {
                log::error!("Error drawing UI: {}", e);
                break;
            }

            // This block ensures all guards are dropped at the end of the scope
            {
                let mut active_tab_guard = self.app_state.active_tab.lock().unwrap();
                let mut selected_index_guard = self.app_state.selected_index.lock().unwrap();
                let mut details_focused_guard = self.app_state.details_focused.lock().unwrap();
                let mut group_focused_field_guard =
                    self.app_state.group_focused_field.lock().unwrap();
                let mut client_focused_field_guard =
                    self.app_state.client_focused_field.lock().unwrap();
                let mut is_editing_guard = self.app_state.is_editing_client_name.lock().unwrap();
                let status_data_guard = self.status_data.lock().unwrap();

                let is_editing = *is_editing_guard;

                let max_items = if let Some(data) = &*status_data_guard {
                    match *active_tab_guard {
                        TabSelection::Groups => data.result.server.groups.len(),
                        TabSelection::Clients => data
                            .result
                            .server
                            .groups
                            .iter()
                            .map(|g| g.clients.len())
                            .sum(),
                        TabSelection::Streams => data.result.server.streams.len(),
                    }
                } else {
                    0
                };

                let input_event = handle_input(
                    &mut active_tab_guard,
                    &mut selected_index_guard,
                    max_items,
                    &mut details_focused_guard,
                    &mut group_focused_field_guard,
                    &mut client_focused_field_guard,
                    is_editing,
                );

                let active_tab = active_tab_guard.clone();
                let details_focused = *details_focused_guard;
                let client_focused_field = client_focused_field_guard.clone();

                match input_event {
                    Ok(InputEvent::Quit) => break,
                    Ok(InputEvent::Edit) => {
                        if active_tab == TabSelection::Clients
                            && details_focused
                            && client_focused_field == ClientDetailsFocus::Name
                        {
                            if !*is_editing_guard {
                                *is_editing_guard = true;
                                let mut editing_name =
                                    self.app_state.editing_client_name.lock().unwrap();
                                if let Some(data) = &*status_data_guard {
                                    let mut client_count = 0;
                                    'outer: for group in &data.result.server.groups {
                                        for client in &group.clients {
                                            if client_count == *selected_index_guard {
                                                *editing_name = client.config.name.clone();
                                                break 'outer;
                                            }
                                            client_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(InputEvent::Confirm) => {
                        if *is_editing_guard {
                            *is_editing_guard = false;
                            if let Some(data) = &*status_data_guard {
                                let mut client_count = 0;
                                'outer: for group in &data.result.server.groups {
                                    for client in &group.clients {
                                        if client_count == *selected_index_guard {
                                            let client_id = client.id.clone();
                                            let new_name = self
                                                .app_state
                                                .editing_client_name
                                                .lock()
                                                .unwrap()
                                                .clone();
                                            let set_name_request = crate::commands::client::setname::create_set_name_request(
                                                &client_id, &new_name,
                                            );
                                            if let Err(e) =
                                                self.cmd_tx.try_send(set_name_request)
                                            {
                                                log::error!(
                                                    "Failed to send set name command: {}",
                                                    e
                                                );
                                            }
                                            break 'outer;
                                        }
                                        client_count += 1;
                                    }
                                }
                            }
                        }
                    }
                    Ok(InputEvent::Cancel) => {
                        if *is_editing_guard {
                            *is_editing_guard = false;
                        }
                    }
                    Ok(InputEvent::Char(c)) => {
                        if is_editing {
                            self.app_state.editing_client_name.lock().unwrap().push(c);
                        }
                    }
                    Ok(InputEvent::Backspace) => {
                        if is_editing {
                            self.app_state.editing_client_name.lock().unwrap().pop();
                        }
                    }
                    Ok(InputEvent::TabChanged(_)) => {
                        if *is_editing_guard {
                            *is_editing_guard = false;
                        }
                        *details_focused_guard = false;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Error handling input: {}", e);
                        break;
                    }
                }
            }

            check_auto_focus(
                &mut self.app_state.details_focused.lock().unwrap(),
                &self.app_state.active_tab.lock().unwrap(),
                &mut self.app_state.group_focused_field.lock().unwrap(),
                &mut self.app_state.client_focused_field.lock().unwrap(),
                &self.status_data.lock().unwrap(),
            );

            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        restore_terminal()
    }
}