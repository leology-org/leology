use core::str::FromStr;
use snarkvm::console::{
    account::PrivateKey,
    network::MainnetV0 as Nw,
    prelude::{Environment, Uniform},
    types::Field,
};

use anyhow::{anyhow, Result};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use crate::Account;

/// Creates a new account with the given seed
pub fn new_account(seed: String) -> Result<Account<Nw>> {
    let seed = Field::new(
        <Nw as Environment>::Field::from_str(&seed)
            .map_err(|e| anyhow!("Invalid seed - {e}"))?,
    );
    let private_key = PrivateKey::try_from(seed)
        .map_err(|_| anyhow!("Failed to convert the seed into a valid private key"))?;
    let account = Account::<Nw>::try_from(private_key)?;
    Ok(account)
}
