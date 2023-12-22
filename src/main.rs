mod ascii_art;
mod client;
mod config;
mod daemon;

use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, help = "Run as daemon")]
    daemon: bool,
    #[arg(long, short, help = "Set the socket path for the client or daemon")]
    socket_path: Option<String>,
    #[arg(
        long,
        short,
        help = "On exit print the execution time, for benchmarking"
    )]
    benchmark: bool,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
lazy_static! {
    static ref SOCKET_PATH: String = parse_path().unwrap_or("/tmp/hayabusa".to_string());
}

#[cfg(target_os = "windows")]
lazy_static! {
    static ref SOCKET_PATH: String = parse_path().unwrap_or("hayabusa".to_string());
}
fn parse_path() -> Option<String> {
    Args::parse().socket_path
}

#[tokio::main]
async fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    let args: Args = Args::parse();
    match args.daemon {
        true => daemon::main::main().await,
        //man, I don't remember why one is async and one isn't, but I'll figure that out another time
        false => client::main::main(),
    }
    if args.benchmark {
        println!("Execution time: {:?}", start.elapsed());
    }
}
