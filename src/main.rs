mod input;
mod websocket;
mod ui;
mod commands;
mod cli;
mod app;

use std::io::Result;
use tokio::sync::mpsc;
use cli::Args;
use clap::Parser;  // Added this import

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Handle version flag
    if args.version {
        println!("{}", cli::version());
        return Ok(());
    }

    // Create application
    let mut app = app::Application::new()?;

    // Create channels
    let (tx, message_rx) = mpsc::channel::<String>(32);
    let (status_tx, status_rx) = mpsc::channel::<websocket::ConnectionStatus>(32);

    // Start WebSocket connection
    tokio::spawn(async move {
        websocket::websocket_task(tx, status_tx, args.host, args.port).await;
    });

    // Run the application
    app.run(message_rx, status_rx).await?;

    Ok(())
}