use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::io::Result;
use crate::websocket::ConnectionStatus;
use crate::ui::{initialize_terminal, restore_terminal, draw_ui, AppState};
use crate::input::{handle_input, InputEvent};
use std::time::Duration;
use crate::models::status::SnapcastStatus;

pub struct Application {
    pub terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    pub app_state: AppState,
    pub status_data: Arc<Mutex<Option<SnapcastStatus>>>,
}

impl Application {
    pub fn new() -> Result<Self> {
        let terminal = initialize_terminal()?;

        let app_state = AppState {
            last_message: Arc::new(Mutex::new(String::from("Waiting for messages..."))),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
        };

        let status_data = Arc::new(Mutex::new(None));

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
        let status_arc = self.app_state.status.clone();
        tokio::spawn(async move {
            while let Some(new_status) = status_rx.recv().await {
                *status_arc.lock().unwrap() = new_status;
            }
        });

        // Start status data update task
        let status_data_arc = self.status_data.clone();
        tokio::spawn(async move {
            while let Some(msg) = message_rx.recv().await {
                if let Ok(status) = crate::commands::server::getstatus::parse_status_response(&msg) {
                    *status_data_arc.lock().unwrap() = Some(status);
                }
            }
        });

        loop {
            // Get the server version from status_data
            let server_version = {
                let status_data = self.status_data.lock().unwrap();
                status_data.as_ref().and_then(|data| Some(data.result.server.snapserver.version.clone())).unwrap_or_default()
            };

            // Draw UI
            if let Err(e) = draw_ui(&mut self.terminal, &self.app_state, &server_version) {
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