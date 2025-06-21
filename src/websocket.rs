use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use tungstenite::protocol::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
use std::time::Duration;
use std::fmt;

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

pub async fn websocket_task(tx: mpsc::Sender<String>, status_tx: mpsc::Sender<ConnectionStatus>) {
    let websocket_url = "ws://localhost:1780/jsonrpc";

    loop {
        let _ = status_tx.send(ConnectionStatus::Connecting).await;

        match connect_async(websocket_url).await {
            Ok((ws_stream, _)) => {
                let _ = status_tx.send(ConnectionStatus::Connected).await;

                if let Err(e) = handle_connection(ws_stream, &tx).await {
                    let _ = status_tx.send(ConnectionStatus::Error(e.to_string())).await;
                }
            }
            Err(e) => {
                let _ = status_tx.send(ConnectionStatus::Error(e.to_string())).await;
            }
        }

        let _ = status_tx.send(ConnectionStatus::Disconnected).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn handle_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
    tx: &mpsc::Sender<String>,
) -> Result<(), String> {
    let (mut write, mut read) = ws_stream.split();

    if let Err(e) = write.send(Message::Ping(vec![])).await {
        return Err(format!("Failed to send ping: {}", e));
    }

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if tx.send(text).await.is_err() {
                    return Err("Failed to send message to channel".into());
                }
            }
            Ok(Message::Ping(_)) => {
                if let Err(e) = write.send(Message::Pong(vec![])).await {
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