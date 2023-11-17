use core::str::FromStr;
use std::process::Command;

use rand::SeedableRng;
use snarkvm::console::{
    account::PrivateKey,
    network::Testnet3,
    prelude::{Environment, Uniform},
    types::Field,
};

use anyhow::{anyhow, Result};
use rand_chacha::ChaChaRng;

type Network = Testnet3;

pub fn devnet() -> std::io::Result<()> {
    // Execute the bash script using the Command module
    let status = Command::new("bash").arg("devnet.sh").status()?;

    // Check if the script executed successfully
    if status.success() {
        println!("Script executed successfully");
    } else {
        println!("Script execution failed");
    }

    Ok(())
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
