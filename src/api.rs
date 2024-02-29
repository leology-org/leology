use anyhow::{anyhow, Result};
use rand::{rngs::StdRng, SeedableRng};
use snarkvm::console::{
    account::PrivateKey,
    network::Testnet3 as Nw,
    prelude::{Environment, Uniform},
    types::Field,
};

use crate::Account;
use std::str::FromStr;

/// Generates a new account, optionally using a provided seed for deterministic results.
///
/// # Arguments
///
/// * `seed`: An optional string seed to deterministically generate a private key.
///
/// # Returns
///
/// This function returns an `Account<Nw>`, which encapsulates the private key and address on the Aleo network.
///
/// # Errors
///
/// If the seed provided is invalid or the private key cannot be generated, an error will be returned.
pub fn new_account(seed: Option<String>) -> Result<Account<Nw>> {
    let seed = match seed {
        Some(seed_str) => {
            // Convert the provided seed string into a field element, returning an error if invalid.
            let seed_field =
                <Nw as Environment>::Field::from_str(&seed_str).map_err(|e| anyhow!("Invalid seed: {e}"))?;
            Field::new(seed_field)
        }
        None => {
            // If no seed is provided, generate a random field element.
            Field::rand(&mut StdRng::from_entropy())
        }
    };

    // Attempt to construct a private key from the seed.
    let private_key =
        PrivateKey::try_from(seed).map_err(|_| anyhow!("Failed to convert seed into a valid private key"))?;

    // Create an account using the private key.
    let account = Account::<Nw>::try_from(private_key)?;

    Ok(account)
}
