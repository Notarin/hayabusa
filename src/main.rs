#![forbid(unsafe_code)]
mod daemon;
mod fetch_info;
mod fetch;

use std::sync::Mutex;
use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, help = "Run as daemon")]
    daemon: bool,
}

#[cfg(target_os = "linux")]
lazy_static!(
    static ref SOCKET_PATH: Mutex<String> = Mutex::new("/tmp/hayabusa".to_string());
);

#[cfg(target_os = "windows")]
lazy_static!(
    static ref SOCKET_PATH: Mutex<String> = Mutex::new("hayabusa".to_string());
);

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    //daemon mode is the system service that tracks system info
    if args.daemon {
        println!("Running as daemon");
        daemon::main().await;
    } else {
        fetch::main();
    }
}
