#![forbid(unsafe_code)]

use crate::*;
use colored::*;
use std::fmt::{self};

/// Represents an Aleo account with associated keys and address.
///
/// An `Account` is a fundamental entity in Aleo that holds cryptographic keys
/// for signing transactions and interacting with the network.
#[derive(Clone, Debug)]
pub struct Account<N: Network> {
    /// The private key associated with the account, used for signing transactions.
    private_key: PrivateKey<N>,
    /// The view key derived from the private key, used for certain privacy-preserving operations.
    view_key: ViewKey<N>,
    /// The public address of the account, used for receiving transactions.
    address: Address<N>,
}

impl<N: Network> Account<N> {
    /// Generates a new `Account` with a random private key.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator implementing the `Rng` and `CryptoRng` traits.
    ///
    /// # Errors
    ///
    /// Returns an error if the generation of the private key fails.
    pub fn new<R: Rng + CryptoRng>(rng: &mut R) -> Result<Self> {
        Self::try_from(PrivateKey::new(rng)?)
    }

    /// Returns a reference to the account's private key.
    pub const fn private_key(&self) -> &PrivateKey<N> {
        &self.private_key
    }

    /// Returns a reference to the account's view key.
    pub const fn view_key(&self) -> &ViewKey<N> {
        &self.view_key
    }

    /// Returns the account's address.
    pub const fn address(&self) -> Address<N> {
        self.address
    }
}

impl<N: Network> Account<N> {
    /// Signs a given message using the account's private key.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to sign, represented as an array of field elements.
    /// * `rng` - A random number generator implementing the `Rng` and `CryptoRng` traits.
    ///
    /// # Errors
    ///
    /// Returns an error if the signing operation fails.
    pub fn sign<R: Rng + CryptoRng>(&self, message: &[Field<N>], rng: &mut R) -> Result<Signature<N>> {
        Signature::sign(&self.private_key, message, rng)
    }

    /// Signs a given message as bytes using the account's private key.
    pub fn sign_bytes<R: Rng + CryptoRng>(&self, message: &[u8], rng: &mut R) -> Result<Signature<N>> {
        Signature::sign_bytes(&self.private_key, message, rng)
    }

    /// Signs a given message as bits using the account's private key.
    pub fn sign_bits<R: Rng + CryptoRng>(&self, message: &[bool], rng: &mut R) -> Result<Signature<N>> {
        Signature::sign_bits(&self.private_key, message, rng)
    }

    /// Verifies a signature for a given message using the account's address.
    pub fn verify(&self, message: &[Field<N>], signature: &Signature<N>) -> bool {
        signature.verify(&self.address, message)
    }

    /// Verifies a signature for a given message as bytes using the account's address.
    pub fn verify_bytes(&self, message: &[u8], signature: &Signature<N>) -> bool {
        signature.verify_bytes(&self.address, message)
    }

    /// Verifies a signature for a given message as bits using the account's address.
    pub fn verify_bits(&self, message: &[bool], signature: &Signature<N>) -> bool {
        signature.verify_bits(&self.address, message)
    }
}

// Implementations for creating `Account` from `PrivateKey` and string representations.
impl<N: Network> TryFrom<PrivateKey<N>> for Account<N> {
    type Error = Error;

    fn try_from(private_key: PrivateKey<N>) -> Result<Self, Self::Error> {
        Self::try_from(&private_key)
    }
}

impl<N: Network> TryFrom<&PrivateKey<N>> for Account<N> {
    type Error = Error;

    fn try_from(private_key: &PrivateKey<N>) -> Result<Self, Self::Error> {
        let view_key = ViewKey::try_from(private_key)?;
        let address = view_key.to_address();
        Ok(Self { private_key: *private_key, view_key, address })
    }
}

impl<N: Network> TryFrom<String> for Account<N> {
    type Error = Error;

    fn try_from(private_key: String) -> Result<Self, Self::Error> {
        Self::try_from(&private_key)
    }
}

impl<N: Network> TryFrom<&String> for Account<N> {
    type Error = Error;

    fn try_from(private_key: &String) -> Result<Self, Self::Error> {
        Self::from_str(private_key.as_str())
    }
}

impl<N: Network> TryFrom<&str> for Account<N> {
    type Error = Error;

    fn try_from(private_key: &str) -> Result<Self, Self::Error> {
        Self::from_str(private_key)
    }
}

impl<N: Network> FromStr for Account<N> {
    type Err = Error;

    fn from_str(private_key: &str) -> Result<Self, Self::Err> {
        Self::try_from(PrivateKey::from_str(private_key)?)
    }
}

impl<N: Network> Display for Account<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " {:>12}  {}\n {:>12}  {}\n {:>12}  {}",
            "Private Key".cyan().bold(),
            self.private_key,
            "View Key".cyan().bold(),
            self.view_key,
            "Address".cyan().bold(),
            self.address
        )
    }
}

// Unit tests to ensure functionality works as expected.
#[cfg(test)]
mod tests {
    use super::*;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_sign() {
        let mut rng = TestRng::default();
        let account = Account::<CurrentNetwork>::new(&mut rng).expect("Failed to create account");
        let message = vec![Field::rand(&mut rng); 10];

        let signature = account.sign(&message, &mut rng).expect("Failed to sign message");
        assert!(account.verify(&message, &signature), "Signature verification failed");
    }

    #[test]
    fn test_sign_bytes() {
        let mut rng = TestRng::default();
        let account = Account::<CurrentNetwork>::new(&mut rng).expect("Failed to create account");
        let message = (0..10).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        let signature = account.sign_bytes(&message, &mut rng).expect("Failed to sign message bytes");
        assert!(account.verify_bytes(&message, &signature), "Byte signature verification failed");
    }

    #[test]
    fn test_sign_bits() {
        let mut rng = TestRng::default();
        let account = Account::<CurrentNetwork>::new(&mut rng).expect("Failed to create account");
        let message = (0..10).map(|_| rng.gen::<bool>()).collect::<Vec<bool>>();

        let signature = account.sign_bits(&message, &mut rng).expect("Failed to sign message bits");
        assert!(account.verify_bits(&message, &signature), "Bit signature verification failed");
    }
}
