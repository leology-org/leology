pub mod engine;
pub mod core;

use clap::{Parser, Subcommand};
use std::future::Future;
use tokio::runtime::Runtime;

/// Executes the given future using a new Tokio runtime.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().expect("Failed to start Tokio runtime");
    rt.block_on(future)
}

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
    Test,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Test => {
            println!("Running tests...");
            let outcome = block_on(run_tests());
            ensure_ok(outcome);
        }
    }
}

// Placeholder for the asynchronous test execution function.
// This function should be replaced with your actual test logic.
async fn run_tests() -> Result<(), String> {
    // Test execution logic goes here.
    Ok(())
}

fn ensure_ok(result: Result<(), String>) {
    match result {
        Ok(_) => println!("Tests completed successfully."),
        Err(e) => {
            println!("Error running tests: {}", e);
            std::process::exit(1);
        }
    }
}
