use clap::{Parser, Subcommand};

use crate::core::{devnet_start, devnet_stop};

mod core;

/// Command Line Interface for the Leology Test Framework.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Subcommands for the CLI.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Subcommand to start the local devnet
    Start,
    /// Subcommand to stop the local devnet
    Stop,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Start => {
            println!("Starting the blockchain...");
            devnet_start().expect("Failure starting the devnet");
        }
        Commands::Stop => {
            println!("Stopping the blockchain...");
            devnet_stop().expect("Failure stopping the devnet");
        }
    }
}
