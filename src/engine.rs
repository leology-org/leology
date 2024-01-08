use snarkvm::circuit::AleoV0 as Aleo;
use snarkvm::ledger::block::Transaction;
use snarkvm::package::Package;
use snarkvm::prelude::*;

const SNARKVM_CONTRACTS_BUILD_FOLDER: &str = "build";
const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:3030";

type Result<T> = core::result::Result<T, Box<dyn error::Error>>;

/// A smart contract's function definition.
/// It is composed of the function identifier and its arguments.
pub struct FunctionDef {
    function: Identifier<Testnet3>,
    inputs: Vec<Value<Testnet3>>,
}

impl FunctionDef {
    /// Attempts to create a `FucntionDef` from its identifier and arguments
    pub fn try_from(name: &str, args: Vec<&str>) -> Result<Self> {
        Ok(Self {
            function: Identifier::from_str(name)?,
            inputs: args
                .iter()
                .map(|arg| Value::from_str(arg).unwrap())
                .collect(),
        })
    }
}

/// Main component to interact with smart contracts
pub struct Engine {
    package: Package<Testnet3>,
}

impl Engine {
    /// Attempts to load the engine from the ./contracts folder.
    /// The ./contracts/build folder must be present as well with the compiled code.
    pub fn try_load() -> Result<Self> {
        // load the package from the ./contracts folder
        let full_path = format!("{}",SNARKVM_CONTRACTS_BUILD_FOLDER);
        let package = Package::open(full_path.as_ref())?;

        Ok(Self { package })
    }

    /// Executes a given `FunctionDef`.
    /// The private key of the sender must be supplied.
    /// The response and transaction objects are returned.
    pub fn execute(
        &self,
        def: FunctionDef,
        private_key_str: &str,
    ) -> Result<(Response<Testnet3>, Transaction<Testnet3>)> {
        // load the private key
        let private_key = PrivateKey::<Testnet3>::from_str(private_key_str)?;

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
