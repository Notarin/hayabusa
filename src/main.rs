mod daemon;
mod ascii_art;
mod client;
mod config;

use clap::Parser;
use lazy_static::lazy_static;
use crate::client::main;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, help = "Run as daemon")]
    daemon: bool,
    #[arg(long, short, help = "On exit print the execution time, for benchmarking")]
    benchmark: bool,
}

#[cfg(target_os = "linux")]
lazy_static!(
    static ref SOCKET_PATH: String = "/tmp/hayabusa".to_string();
);

#[cfg(target_os = "windows")]
lazy_static!(
    static ref SOCKET_PATH: String = "hayabusa".to_string();
);

#[tokio::main]
async fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    let args: Args = Args::parse();
    //daemon mode is the system service that tracks system info
    if args.daemon {
        println!("Running as daemon");
        daemon::main::main().await;
    } else {
        main::main();
    }
    if args.benchmark {
        println!("Execution time: {:?}", start.elapsed());
    }
}
