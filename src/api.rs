use std::process::Command;
use core::str::FromStr;
use snarkvm::console::{
    account::PrivateKey,
    prelude::{Environment, Uniform},
    network::MainnetV0 as Nw,
    types::Field,
};


use anyhow::{anyhow, Result};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;


use crate::*;

/// Creates a new account with the given seed
pub fn new_account(seed: Option<String>) -> Result<Account<Nw>> {
    // Recover the seed.
    let seed = match seed {
        // Recover the field element deterministically.
        Some(seed) => Field::new(
            <Nw as Environment>::Field::from_str(&seed)
                .map_err(|e| anyhow!("Invalid seed - {e}"))?,
        ),
        // Sample a random field element.
        None => Field::rand(&mut ChaChaRng::from_entropy()),
    };
    // Recover the private key from the seed as a field element.
    let private_key = PrivateKey::try_from(seed)
        .map_err(|_| anyhow!("Failed to convert the seed into a valid private key"))?;
    // Construct the account.
    let account = Account::<Nw>::try_from(private_key)?;
    Ok(account)
}

