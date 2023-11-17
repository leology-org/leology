use core::str::FromStr;
use std::process::Command;

use anyhow::{anyhow, Result};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use snarkvm::console::{
    account::PrivateKey,
    network::Testnet3,
    prelude::{Environment, Uniform},
    types::Field,
};

type Network = Testnet3;

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

pub fn devnet_start() -> std::io::Result<()> {
    execute_script("start.sh")
}

pub fn devnet_stop() -> std::io::Result<()> {
    execute_script("stop.sh")
}

pub fn new_account(seed: Option<String>) -> Result<snarkos_account::Account<Testnet3>> {
    // Recover the seed.
    let seed = match seed {
        // Recover the field element deterministically.
        Some(seed) => Field::new(
            <Network as Environment>::Field::from_str(&seed)
                .map_err(|e| anyhow!("Invalid seed - {e}"))?,
        ),
        // Sample a random field element.
        None => Field::rand(&mut ChaChaRng::from_entropy()),
    };
    // Recover the private key from the seed as a field element.
    let private_key = PrivateKey::try_from(seed)
        .map_err(|_| anyhow!("Failed to convert the seed into a valid private key"))?;
    // Construct the account.
    let account = snarkos_account::Account::<Network>::try_from(private_key)?;
    Ok(account)
}
