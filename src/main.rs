use std::future::Future;

use clap::{Parser, Subcommand};

use crate::core::devnet;

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
    /// Subcommand to run tests.
    Node,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Node => {
            println!("Starting the blockchain...");
            devnet().expect("Failure starting the devnet");
        }
    }
}
