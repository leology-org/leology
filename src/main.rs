use clap::{Parser, Subcommand};
use std::{
    io::{self, ErrorKind},
    process::Command,
};

/// Command Line Interface for the Leology Test Framework.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Start,
    Stop,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Start => {
            println!("Starting the blockchain...");
            execute_script("start.sh")?;
        }
        Commands::Stop => {
            println!("Stopping the blockchain...");
            execute_script("stop.sh")?;
        }
    }

    Ok(())
}

fn execute_script(script: &str) -> io::Result<()> {
    match Command::new("bash").arg(script).status() {
        Ok(status) if status.success() => {
            println!("{} executed successfully.", script);
            Ok(())
        }
        Ok(_) => {
            eprintln!("{} failed to execute.", script);
            Err(io::Error::new(ErrorKind::Other, "Script execution failed"))
        }
        Err(e) => {
            eprintln!("Failed to start the script {}: {}", script, e);
            Err(e)
        }
    }
}
