use tokio::sync::mpsc;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::connect_async;
use std::time::Duration;
use std::fmt;
use crate::commands::server::getstatus::create_status_request;
use log;

#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Error(String),
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Connecting => write!(f, "Connecting..."),
            ConnectionStatus::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

pub async fn websocket_task(
    tx: mpsc::Sender<String>,
    mut cmd_rx: mpsc::Receiver<String>,
    status_tx: mpsc::Sender<ConnectionStatus>,
    host: String,
    port: u16,
) {
    let websocket_url = format!("ws://{}:{}/jsonrpc", host, port);
    let reconnect_delay = Duration::from_secs(1);

    loop {
        update_status(&status_tx, ConnectionStatus::Connecting).await;

        match connect_async(&websocket_url).await {
            Ok((ws_stream, _)) => {
                update_status(&status_tx, ConnectionStatus::Connected).await;

                let (mut write, mut read) = ws_stream.split();

                if let Err(e) = write.send(Message::Ping(vec![].into())).await {
                    log::debug!("Failed to send ping: {}", e);
                }

                let status_request = create_status_request();
                if let Err(e) = write.send(Message::Text(status_request.into())).await {
                    log::debug!("Failed to send status request: {}", e);
                }

                loop {
                    tokio::select! {
                        Some(msg) = read.next() => {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if tx.send(text.to_string()).await.is_err() {
                                        log::debug!("Failed to send message to channel");
                                        break;
                                    }

                                    if is_notification(&text) {
                                        let status_request = create_status_request();
                                        if let Err(e) = write.send(Message::Text(status_request.into())).await {
                                            log::debug!("Failed to send status request after notification: {}", e);
                                            break;
                                        }
                                    }
                                }
                                Ok(Message::Ping(data)) => {
                                    if let Err(e) = write.send(Message::Pong(data)).await {
                                        log::debug!("Failed to send pong: {}", e);
                                        break;
                                    }
                                }
                                Ok(Message::Close(_)) => {
                                    log::debug!("WebSocket closed by server");
                                    break;
                                }
                                Err(e) => {
                                    log::debug!("WebSocket read error: {}", e);
                                    break;
                                }
                                _ => continue,
                            }
                        }
                        Some(cmd) = cmd_rx.recv() => {
                            if let Err(e) = write.send(Message::Text(cmd.into())).await {
                                log::debug!("Failed to send command: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::debug!("Connection failed: {}", e);
                update_status(&status_tx, ConnectionStatus::Error(e.to_string())).await;
            }
        }

        update_status(&status_tx, ConnectionStatus::Disconnected).await;
        tokio::time::sleep(reconnect_delay).await;
    }
}

async fn update_status(status_tx: &mpsc::Sender<ConnectionStatus>, status: ConnectionStatus) {
    if let Err(e) = status_tx.send(status).await {
        log::error!("Failed to send status update: {}", e);
    }
}

fn is_notification(message: &str) -> bool {
    message.contains("\"method\"") && !message.contains("\"result\"")
}