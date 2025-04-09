use clap::{Parser, Subcommand};
use std::process::Command;

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

/// Starts the devnet chain
pub fn devnet_start() -> std::io::Result<()> {
    execute_script("start.sh")
}

/// Stops the devnet chain
pub fn devnet_stop() -> std::io::Result<()> {
    execute_script("stop.sh")
}
fn execute_script(command: &str) -> std::io::Result<()> {
    // Execute the bash script using the Command module
    let status = Command::new("bash").arg(command).status()?;

    // Check if the script executed successfully
    if status.success() {
        println!("Script executed successfully");
    } else {
        println!("Script execution failed");
    }

    Ok(())
}
