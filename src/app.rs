use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::io::Result;
use crate::websocket::ConnectionStatus;
use crate::ui::{initialize_terminal, restore_terminal, draw_ui, AppState, TabSelection};
use crate::models::server::getstatus::GetStatusData;
use std::time::Duration;
use crate::commands::server::getstatus::extract_server_version;
use crate::ui::GroupDetailsFocus;
use crate::ui::ClientDetailsFocus;
use crate::input::InputEvent;

mod input_handler;

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
            group_focused_field: Arc::new(Mutex::new(GroupDetailsFocus::None)),
            client_focused_field: Arc::new(Mutex::new(ClientDetailsFocus::None)),
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
        let status_arc = Arc::clone(&self.app_state.status);
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
        let ui_status_data: Arc<Mutex<Option<GetStatusData>>> = Arc::clone(&self.app_state.status_data);

        tokio::spawn(async move {
            while let Some(msg) = message_rx.recv().await {
                if msg.starts_with("[INFO") || msg.starts_with("[DEBUG") {
                    continue;
                }

                if is_notification(&msg) {
                    if let Ok(mut message) = message_arc.lock() {
                        *message = msg;
                    }
                    continue;
                }

                match crate::commands::server::getstatus::parse_status_response(&msg) {
                    Ok(status) => {
                        if let Ok(mut data) = status_data_arc.lock() {
                            *data = Some(status.clone());
                        }
                        if let Ok(mut ui_data) = ui_status_data.lock() {
                            *ui_data = Some(status);
                        }

                        if let Some(version) = extract_server_version(&msg) {
                            if let Ok(mut version_lock) = version_arc.lock() {
                                *version_lock = version;
                            }
                        }
                    }
                    Err(_) => {
                        if let Ok(mut message) = message_arc.lock() {
                            *message = format!("Error parsing message");
                        }
                    }
                }
            }
        });

        loop {
            if let Err(e) = draw_ui(&mut self.terminal, &self.app_state) {
                eprintln!("Error drawing UI: {}", e);
                break;
            }

            match input_handler::handle_app_input(
                &mut self.app_state.active_tab.lock().unwrap(),
                &mut self.app_state.selected_index.lock().unwrap(),
                &mut self.app_state.details_focused.lock().unwrap(),
                &mut self.app_state.group_focused_field.lock().unwrap(),
                &mut self.app_state.client_focused_field.lock().unwrap(),
                &self.status_data.lock().unwrap(),
            ) {
                Ok(Some(InputEvent::Quit)) => break,
                Ok(Some(InputEvent::TabChanged(_))) => {},
                Ok(Some(InputEvent::Up)) => {},
                Ok(Some(InputEvent::Down)) => {},
                Ok(Some(InputEvent::CycleFields)) => {},
                Ok(Some(InputEvent::None)) => {},
                Ok(None) => {},
                Err(e) => {
                    log::error!("Error handling input: {}", e);
                    break;
                }
            }

            input_handler::check_auto_focus(
                &mut self.app_state.details_focused.lock().unwrap(),
                &self.app_state.active_tab.lock().unwrap(),
                &mut self.app_state.group_focused_field.lock().unwrap(),
                &mut self.app_state.client_focused_field.lock().unwrap(),
                &self.status_data.lock().unwrap(),
            );

            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        restore_terminal()
    }
}

fn is_notification(message: &str) -> bool {
    message.contains("\"method\"") && !message.contains("\"result\"")
}