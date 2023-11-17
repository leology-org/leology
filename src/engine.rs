use snarkvm::circuit::AleoV0 as Aleo;
use snarkvm::ledger::block::Transaction;
use snarkvm::package::Package;
use snarkvm::prelude::*;
use std::env;

const SNARKVM_CONTRACTS_FOLDER: &str = "contracts";
const PRIVATE_KEY_ENV: &str = "PRIVATE_KEY";
const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:3000";

type Result<T> = core::result::Result<T, Box<dyn error::Error>>;

pub struct FunctionDef {
    function: Identifier<Testnet3>,
    inputs: Vec<Value<Testnet3>>,
}

pub struct Engine {
    package: Package<Testnet3>,
}

impl Engine {
    pub fn try_load() -> Result<Self> {
        // load the package from the ./contracts folder
        let package = Package::open(SNARKVM_CONTRACTS_FOLDER.as_ref())?;

        // perform the build (this might fail)
        package.build::<Aleo>(None)?;

        Ok(Self { package })
    }

    pub fn execute(&self, def: FunctionDef) -> Result<(Response<Testnet3>, Transaction<Testnet3>)> {
        // load the private key
        let private_key_str = env::var(PRIVATE_KEY_ENV)?;
        let private_key = PrivateKey::<Testnet3>::from_str(&*private_key_str)?;

        // Initialize an RNG.
        let rng = &mut rand::thread_rng();

        // Execute the request.
        let (response, execution, _) = self.package.execute::<Aleo, _>(
            DEFAULT_ENDPOINT.parse().unwrap(),
            &private_key,
            def.function,
            &def.inputs,
            rng,
        )?;
        let transaction = Transaction::from_execution(execution, None)?;
        Ok((response, transaction))
    }
}
