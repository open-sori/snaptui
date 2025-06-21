use tokio::sync::mpsc;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::connect_async;
use std::time::Duration;
use std::fmt;
use crate::commands::server::getstatus::create_status_request;

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

                if let Err(_) = write.send(Message::Ping(vec![].into())).await {
                }

                let status_request = create_status_request();
                if let Err(_) = write.send(Message::Text(status_request.into())).await {
                }

                loop {
                    tokio::select! {
                        Some(msg) = read.next() => {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if tx.send(text.to_string()).await.is_err() {
                                        break;
                                    }
                                }
                                Ok(Message::Ping(data)) => {
                                    if let Err(_) = write.send(Message::Pong(data)).await {
                                        break;
                                    }
                                }
                                Ok(Message::Close(_)) => {
                                    break;
                                }
                                Err(_) => {
                                    break;
                                }
                                _ => continue,
                            }
                        }
                        Some(cmd) = cmd_rx.recv() => {
                            if let Err(_) = write.send(Message::Text(cmd.into())).await {
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                update_status(&status_tx, ConnectionStatus::Error(e.to_string())).await;
            }
        }

        update_status(&status_tx, ConnectionStatus::Disconnected).await;
        tokio::time::sleep(reconnect_delay).await;
    }
}

async fn update_status(status_tx: &mpsc::Sender<ConnectionStatus>, status: ConnectionStatus) {
    if let Err(_) = status_tx.send(status).await {
    }
}