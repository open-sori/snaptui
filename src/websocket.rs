use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
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

                if let Err(e) = handle_connection(ws_stream, &tx).await {
                    log::error!("Connection error: {}", e);
                    update_status(&status_tx, ConnectionStatus::Error(e.to_string())).await;
                }
            }
            Err(e) => {
                log::error!("Connection failed: {}", e);
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

async fn handle_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
    tx: &mpsc::Sender<String>,
) -> Result<(), String> {
    let (mut write, mut read) = ws_stream.split();

    // Send initial ping
    if let Err(e) = write.send(Message::Ping(vec![].into())).await {
        return Err(format!("Failed to send ping: {}", e));
    }

    // Create and send the status request
    let status_request = create_status_request();
    if let Err(e) = write.send(Message::Text(status_request.into())).await {
        return Err(format!("Failed to send status request: {}", e));
    }

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if tx.send(text.to_string()).await.is_err() {
                    return Err("Failed to send message to channel".into());
                }
            }
            Ok(Message::Ping(data)) => {
                if let Err(e) = write.send(Message::Pong(data)).await {
                    return Err(format!("Failed to send pong: {}", e));
                }
            }
            Ok(Message::Close(_)) => {
                return Err("WebSocket closed by server".into());
            }
            Err(e) => {
                return Err(format!("WebSocket read error: {}", e));
            }
            _ => continue,
        }
    }

    Ok(())
}