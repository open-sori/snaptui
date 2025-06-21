use tokio::sync::mpsc;
use std::sync::Arc;
use std::io::Result;
use crate::websocket::ConnectionStatus;
use crate::ui::{initialize_terminal, restore_terminal, draw_ui, AppState};
use crate::input::{handle_input, InputEvent};
use std::time::Duration;

pub struct Application {
    pub terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    pub app_state: AppState,
}

impl Application {
    pub fn new() -> Result<Self> {
        let terminal = initialize_terminal()?;

        let app_state = AppState {
            last_message: Arc::new(std::sync::Mutex::new(String::from("Waiting for messages..."))),
            status: Arc::new(std::sync::Mutex::new(ConnectionStatus::Disconnected)),
        };

        Ok(Self {
            terminal,
            app_state,
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

        loop {
            // Draw UI
            if let Err(e) = draw_ui(&mut self.terminal, &self.app_state) {
                eprintln!("Error drawing UI: {}", e);
                break;
            }

            // Check for new messages or input
            tokio::select! {
                // Process messages
                Some(msg) = message_rx.recv() => {
                    self.handle_message(msg).await;
                }
                // Handle input
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    if let Ok(InputEvent::Quit) = handle_input() {
                        break;
                    }
                }
            }
        }

        restore_terminal()
    }

    async fn handle_message(&self, msg: String) {
        // Try to parse the message as JSON and pretty print it
        let formatted_message = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&msg) {
            if let Ok(pretty) = serde_json::to_string_pretty(&parsed) {
                pretty
            } else {
                msg
            }
        } else {
            msg
        };

        // Update the last message
        *self.app_state.last_message.lock().unwrap() = formatted_message;
    }
}