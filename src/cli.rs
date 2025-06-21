use clap::{Parser, ArgAction};
/// A TUI application for monitoring Snapcast servers
#[derive(Parser, Debug)]
#[command(
    author,
    about,
    long_about = None,
    disable_version_flag = true,
)]
pub struct Args {
    /// Show version information
    #[arg(long, action = ArgAction::SetTrue)]
    pub version: bool,

    /// Snapcast server host
    #[arg(
        long,
        env = "SNAPSERVER_HOST",
        default_value = "127.0.0.1"
    )]
    pub host: String,

    /// Snapcast server port
    #[arg(
        long,
        env = "SNAPSERVER_PORT",
        default_value = "1780"
    )]
    pub port: u16,
}

pub fn version() -> String {
    format!("Version: {}", env!("CARGO_PKG_VERSION"))
}
