#![forbid(unsafe_code)]
mod daemon;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    daemon: bool,
}

fn main() {
    let args: Args = Args::parse();
    //daemon mode is the system service that tracks system info
    if args.daemon {
        println!("Running as daemon");
        daemon::main();
    } else {
        println!("Running as normal");
    }
}
