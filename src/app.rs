use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::io::Result;
use crate::websocket::ConnectionStatus;
use crate::ui::{initialize_terminal, restore_terminal, draw_ui, AppState};
use crate::input::{handle_input, InputEvent};
use std::time::Duration;
use crate::models::server::getstatus::GetStatusData;
use crate::commands::server::getstatus::extract_server_version;
use chrono::Local;
use serde_json::Value;

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
    ) -> Result<()> {
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
                match crate::commands::server::getstatus::parse_status_response(&msg) {
                    Ok(status) => {
                        if let Ok(mut data) = status_data_arc.lock() {
                            *data = Some(status.clone());
                        }
                        if let Ok(mut ui_data) = ui_status_data.lock() {
                            *ui_data = Some(status);
                        }

                        // Update the last message with the timestamp
                        if let Ok(mut message) = message_arc.lock() {
                            *message = format!("Last update: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
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
                    Err(e) => {
                        // If parsing as GetStatusData fails, check if it's valid JSON
                        if serde_json::from_str::<Value>(&msg).is_ok() {
                            // It's valid JSON but not matching our expected format
                            if let Ok(mut message) = message_arc.lock() {
                                *message = format!("Received valid JSON but not in expected format: {}", e);
                            }
                            if let Ok(mut version_lock) = version_arc.lock() {
                                *version_lock = String::new();
                            }
                            if let Ok(mut ui_data) = ui_status_data.lock() {
                                *ui_data = None;
                            }
                        } else {
                            // It's not valid JSON at all
                            if let Ok(mut message) = message_arc.lock() {
                                *message = format!("Invalid message received: {}", msg);
                            }
                            if let Ok(mut version_lock) = version_arc.lock() {
                                *version_lock = String::new();
                            }
                            if let Ok(mut ui_data) = ui_status_data.lock() {
                                *ui_data = None;
                            }
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

            // Check for input
            if let Ok(InputEvent::Quit) = handle_input() {
                break;
            }

            // Sleep to control UI update rate
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        restore_terminal()
    }
}