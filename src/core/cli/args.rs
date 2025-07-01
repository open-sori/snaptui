use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(
    author,
    about,
    long_about = None,
    disable_version_flag = true,
)]
pub struct Args {
    #[arg(long, action = ArgAction::SetTrue)]
    pub version: bool,

    #[arg(
        long,
        env = "SNAPSERVER_HOST",
        default_value = "127.0.0.1"
    )]
    pub host: String,

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