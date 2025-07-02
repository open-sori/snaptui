mod core;
mod ui;
mod commands;
mod app;
mod models;

use std::io::Result;
use tokio::sync::mpsc;
use core::cli::args::Args;
use core::websocket::connection::{websocket_task, ConnectionStatus};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.version {
        println!("{}", core::cli::args::version());
        return Ok(());
    }

    let (tx, message_rx) = mpsc::channel::<String>(32);
    let (cmd_tx, cmd_rx) = mpsc::channel::<String>(32);
    let (status_tx, status_rx) = mpsc::channel::<ConnectionStatus>(32);

    tokio::spawn(async move {
        websocket_task(tx, cmd_rx, status_tx, args.host, args.port).await;
    });

    let mut app = app::Application::new(cmd_tx)?;
    app.run(message_rx, status_rx).await?;

    Ok(())
}