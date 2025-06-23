use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::io::Result;
use crate::websocket::ConnectionStatus;
use crate::ui::{initialize_terminal, restore_terminal, draw_ui, AppState, TabSelection};
use crate::input::{handle_input, InputEvent};
use std::time::Duration;
use crate::models::server::getstatus::GetStatusData;
use crate::commands::server::getstatus::extract_server_version;
use chrono::Local;

pub struct Application {
    pub terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    pub app_state: AppState,
    pub status_data: Arc<Mutex<Option<GetStatusData>>>,
}

impl Application {
    pub fn new() -> Result<Self> {
        let terminal = initialize_terminal()?;

        let status_data = Arc::new(Mutex::new(None));

        let app_state = AppState {
            last_message: Arc::new(Mutex::new(String::from("Waiting for messages..."))),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            server_version: Arc::new(Mutex::new(String::new())),
            status_data: Arc::clone(&status_data),
            active_tab: Arc::new(Mutex::new(TabSelection::Groups)),
            selected_index: Arc::new(Mutex::new(0)),
            details_focused: Arc::new(Mutex::new(false)),
            focused_field: Arc::new(Mutex::new(crate::ui::DetailsFocus::None)),
        };

        Ok(Self {
            terminal,
            app_state,
            status_data,
        })
    }

    pub async fn run(
        &mut self,
        mut message_rx: mpsc::Receiver<String>,
        mut status_rx: mpsc::Receiver<ConnectionStatus>,
        refresh_tx: mpsc::Sender<bool>,
    ) -> Result<()> {
        // Create a channel for sending WebSocket messages
        let (ws_tx, _ws_rx) = mpsc::channel::<String>(32);

        // Start status update task
        let status_arc = Arc::clone(&self.app_state.status);
        tokio::spawn(async move {
            while let Some(new_status) = status_rx.recv().await {
                if let Ok(mut status) = status_arc.lock() {
                    *status = new_status;
                }
            }
        });

        // Start status data update task
        let status_data_arc: Arc<Mutex<Option<GetStatusData>>> = Arc::clone(&self.status_data);
        let message_arc = Arc::clone(&self.app_state.last_message);
        let version_arc = Arc::clone(&self.app_state.server_version);
        let ui_status_data: Arc<Mutex<Option<GetStatusData>>> = Arc::clone(&self.app_state.status_data);

        tokio::spawn(async move {
            while let Some(msg) = message_rx.recv().await {
                // Skip messages that are internal log messages
                if msg.starts_with("[INFO") || msg.starts_with("[DEBUG") {
                    continue;
                }

                // First check if this is a notification (which might be different from status updates)
                if is_notification(&msg) {
                    if let Ok(mut message) = message_arc.lock() {
                        *message = format!("[Notification] {} | {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
                    }
                    continue;
                }

                // Then try to parse as GetStatusData
                match crate::commands::server::getstatus::parse_status_response(&msg) {
                    Ok(status) => {
                        if let Ok(mut data) = status_data_arc.lock() {
                            *data = Some(status.clone());
                        }
                        if let Ok(mut ui_data) = ui_status_data.lock() {
                            *ui_data = Some(status);
                        }

                        // Extract and store the server version
                        if let Some(version) = extract_server_version(&msg) {
                            if let Ok(mut version_lock) = version_arc.lock() {
                                *version_lock = version;
                            }
                        } else if let Ok(mut version_lock) = version_arc.lock() {
                            *version_lock = "Unknown version".to_string();
                        }
                    }
                    Err(_) => {
                        // Handle error case
                        if let Ok(mut message) = message_arc.lock() {
                            *message = format!("Error parsing message | {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
                        }
                    }
                }
            }
        });

        loop {
            // Draw UI
            if let Err(e) = draw_ui(&mut self.terminal, &self.app_state) {
                eprintln!("Error drawing UI: {}", e);
                break;
            }

            // Prepare all the locks we need
            let mut current_tab = self.app_state.active_tab.lock().unwrap();
            let mut selected_index = self.app_state.selected_index.lock().unwrap();
            let mut details_focused = self.app_state.details_focused.lock().unwrap();
            let mut focused_field = self.app_state.focused_field.lock().unwrap();

            // Store the values we need to pass to handle_input
            let current_tab_value = current_tab.clone();
            let details_focused_value = *details_focused;
            let focused_field_value = focused_field.clone();

            // Determine the maximum number of items based on the current tab
            let max_items = if let Some(data) = &*self.status_data.lock().unwrap() {
                match *current_tab {
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

            // Call handle_input with the values
            match handle_input(
                &mut current_tab,
                &mut selected_index,
                max_items,
                &mut details_focused,
                &mut focused_field,
            ) {
                Ok(event) => match event {
                    InputEvent::Quit => break,
                    InputEvent::TabChanged(new_tab) => {
                        *current_tab = new_tab;
                    },
                    InputEvent::Up | InputEvent::Down => {
                        // Selection was updated in the function
                    },
                    InputEvent::Refresh => {
                        // Create and send a new status request
                        let status_request = crate::commands::server::getstatus::create_status_request();
                        if let Err(e) = refresh_tx.send(true).await {
                            log::error!("Failed to send refresh request: {}", e);
                        }
                        if let Err(e) = ws_tx.send(status_request).await {
                            log::error!("Failed to send WebSocket message: {}", e);
                        }
                    },
                    InputEvent::Select => {
                        if current_tab_value == TabSelection::Clients {
                            *details_focused = true;
                            *focused_field = crate::ui::DetailsFocus::Volume;
                        }
                    },
                    InputEvent::CycleFields => {
                        if current_tab_value == TabSelection::Clients {
                            *focused_field = match *focused_field {
                                crate::ui::DetailsFocus::Volume => crate::ui::DetailsFocus::Muted,
                                crate::ui::DetailsFocus::Muted => crate::ui::DetailsFocus::Latency,
                                crate::ui::DetailsFocus::Latency => crate::ui::DetailsFocus::Volume,
                                _ => crate::ui::DetailsFocus::Volume,
                            };
                        }
                    },
                    InputEvent::None => {}
                },
                Err(e) => {
                    log::error!("Error handling input: {}", e);
                    break;
    }
            }

            // Sleep to control UI update rate
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        restore_terminal()
    }
}

// Add this helper function to check if a message is a notification
fn is_notification(message: &str) -> bool {
    // Snapcast notifications typically have a "method" field for notifications
    // This is a simple check - you might need to adjust based on actual notification format
    message.contains("\"method\"") && !message.contains("\"result\"")
}