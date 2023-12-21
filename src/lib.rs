//! Leology is an Aleo testing framework that makes developing Leo programs faster, easier, and safer. It includes the most commonly used RPC functions and can be run deterministically to make development a breeze.
//!
//! - Easy setup of Local Testnet
//! - Generate Accounts for testing
//! - Programmatic use in Rust to run tests
//! - Testing of both private and public values
//! - Local deployment of programs
//! - Aleo RPC support
//!

pub mod core;
pub mod engine;
pub use snarkos_account::Account;
pub use snarkvm::console::network::Testnet3;
