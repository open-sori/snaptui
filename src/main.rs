mod input;
mod websocket;
mod ui;

use tokio::sync::mpsc;
use std::time::Duration;
use std::io::Result;
use std::sync::Arc;
use websocket::ConnectionStatus;
use ui::{initialize_terminal, restore_terminal, draw_ui, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize terminal
    let mut terminal = initialize_terminal()?;

    // Create application state
    let app_state = AppState {
        messages: Arc::new(std::sync::Mutex::new(std::collections::VecDeque::<String>::new())),
        status: Arc::new(std::sync::Mutex::new(ConnectionStatus::Disconnected)),
    };

    // Channels
    let (tx, mut rx) = mpsc::channel::<String>(32);
    let (status_tx, mut status_rx) = mpsc::channel::<ConnectionStatus>(32);

    // Start status update task
    tokio::spawn({
        let status_arc = app_state.status.clone();
        async move {
            while let Some(new_status) = status_rx.recv().await {
                *status_arc.lock().unwrap() = new_status;
            }
        }
    });

    // Start WebSocket connection
    tokio::spawn(async move {
        websocket::websocket_task(tx, status_tx).await;
    });

    // Main loop
    loop {
        // Draw UI
        if let Err(e) = draw_ui(&mut terminal, &app_state) {
            eprintln!("Error drawing UI: {}", e);
            break;
        }

        // Check for new messages or input
        tokio::select! {
            // Process messages
            Some(msg) = rx.recv() => {
                app_state.messages.lock().unwrap().push_back(msg);
                if app_state.messages.lock().unwrap().len() > 100 {
                    app_state.messages.lock().unwrap().pop_front();
                }
            }
            // Handle input
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                match input::handle_input() {
                    Ok(input::InputEvent::Quit) => break,
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error handling input: {}", e);
                        break;
                    }
                }
            }
        }
    }

    // Cleanup
    restore_terminal()?;
    Ok(())
}