mod core;
mod ui;
mod commands;
mod app;
mod models;

use std::fs::File;
use std::io::Result;
use tokio::sync::mpsc;
use core::cli::args::Args;
use core::websocket::connection::{websocket_task, ConnectionStatus};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // --- Start of new logging setup ---
    let log_file = File::create("snaptui.log").expect("Failed to create log file.");
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(log_file))) // Direct logs to the file
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_micros() // Add precise timestamps for debugging
        .init();
    // --- End of new logging setup ---

    log::info!("Application starting up...");

    let args = Args::parse();

    if args.version {
        println!("{}", core::cli::args::version());
        return Ok(());
    }

    let (tx, message_rx) = mpsc::channel::<String>(32);
    let (cmd_tx, cmd_rx) = mpsc::channel::<String>(32);
    let (status_tx, status_rx) = mpsc::channel::<ConnectionStatus>(32);

    log::info!("Spawning websocket task.");
    tokio::spawn(async move {
        websocket_task(tx, cmd_rx, status_tx, args.host, args.port).await;
    });

    let mut app = app::Application::new(cmd_tx)?;
    log::info!("Starting application run loop.");
    app.run(message_rx, status_rx).await?;

    log::info!("Application shutting down.");
    Ok(())
}