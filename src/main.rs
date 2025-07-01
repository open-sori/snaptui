mod core;
mod websocket;
mod ui;
mod commands;
mod app;
mod models;

use std::io::Result;
use tokio::sync::mpsc;
use core::cli::args::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp(None)
        .filter_module("snaptui::websocket", log::LevelFilter::Warn)
        .filter_module("tungstenite", log::LevelFilter::Warn)
        .init();

    let args = Args::parse();

    if args.version {
        println!("{}", core::cli::args::version());
        return Ok(());
    }

    let mut app = app::Application::new()?;
    let (tx, message_rx) = mpsc::channel::<String>(32);
    let (status_tx, status_rx) = mpsc::channel::<websocket::ConnectionStatus>(32);

    tokio::spawn(async move {
        websocket::websocket_task(tx, status_tx, args.host, args.port).await;
    });

    app.run(message_rx, status_rx).await?;

    Ok(())
}
