pub use rand;
pub use ureq;
pub use indexmap::IndexMap;
pub use snarkvm::ledger::store::helpers::memory::ConsensusMemory;
pub use snarkvm::ledger::store::ConsensusStore;
pub use snarkvm::ledger::store::ConsensusStorage;
pub use snarkvm::ledger::query::*;
pub use snarkvm::console::program::*;
pub use snarkvm::circuit::AleoV0;
pub use aleo_std::StorageMode;
pub use std::path::{Path, PathBuf};
use crate::*;


pub trait ToValue<N: Network> {
    fn to_value(&self) -> Value<N>;
}
impl ToValue<Nw> for u64 {
    fn to_value(&self) -> Value<Nw> {
        Value::from(Literal::U64(U64::new(*self)))
    }
}
impl ToValue<Nw> for Address<Nw> {
    fn to_value(&self) -> Value<Nw> {
        Value::from(Literal::Address(*self))
    }
}
impl ToValue<Nw> for Entry<Nw, Plaintext<Nw>> {
    fn to_value(&self) -> Value<Nw> {
        match self {
            Entry::Public(entry) |
            Entry::Private(entry) |
            Entry::Constant(entry) => {
                Value::Plaintext(entry.clone())
            },
        }
    }
}
pub trait FromValue<N: Network> {
    fn from_value(value: Value<N>) -> Self;
}
impl FromValue<Nw> for Record<Nw, Plaintext<Nw>> {
    fn from_value(value: Value<Nw>) -> Self {
        match value {
            Value::Record(value) => value,
            _ => panic!("Wrong type."),
        }
    }
}
impl FromValue<Nw> for u64 {
    fn from_value(value: Value<Nw>) -> Self {
        match value {
            Value::Plaintext(Plaintext::Literal(Literal::U64(v), _)) => *v,
            _ => panic!("Wrong type."),
        }
    }
}
impl FromValue<Nw> for Address<Nw> {
    fn from_value(value: Value<Nw>) -> Self {
        match value {
            Value::Plaintext(Plaintext::Literal(Literal::Address(v), _)) => v,
            _ => panic!("Wrong type."),
        }
    }
}

/// A helper function to recursively load the program and all of its imports into the process.
pub fn load_program(
    endpoint: &str,
    process: &mut Process<Nw>,
    program_id: &ProgramID<Nw>,
) -> Result<()> {
    // Fetch the program.
    let program = fetch_program(program_id, endpoint)?;

    // Return early if the program is already loaded.
    if process.contains_program(program.id()) {
        return Ok(());
    }

    // Iterate through the program imports.
    for import_program_id in program.imports().keys() {
        // Add the imports to the process if does not exist yet.
        if !process.contains_program(import_program_id) {
            // Recursively load the program and its imports.
            load_program(endpoint, process, import_program_id)?;
        }
    }

    // Add the program to the process if it does not already exist.
    if !process.contains_program(program.id()) {
        process.add_program(&program)?;
    }

    Ok(())
}
/// Fetch the program from the given endpoint.
pub fn fetch_program(program_id: &ProgramID<Nw>, endpoint: &str) -> Result<Program<Nw>> {
    // Send a request to the query node.
    let response = ureq::get(&format!("{endpoint}/mainnet/program/{program_id}")).call();

    // Deserialize the program.
    match response {
        Ok(response) => response.into_json().map_err(|err| err.into()),
        Err(err) => match err {
            ureq::Error::Status(_status, response) => {
                bail!(response.into_string().unwrap_or("Response too large!".to_owned()))
            }
            err => bail!(err),
        },
    }
}

pub fn initialize_vm() -> Result<VM<Nw, ConsensusMemory<Nw>>> {
    let store = ConsensusStore::<Nw, ConsensusMemory<Nw>>::open(Some(3))?;
    let vm = VM::from(store)?;
    Ok(vm)
}
fn broadcast_transaction(transaction: Transaction<Nw>) -> Result<String> {
    let transaction_id = transaction.id();
    ensure!(!transaction.is_fee(), "The transaction is a fee transaction and cannot be broadcast");
    // Send the deployment request to the local development node.
    match ureq::post("http://127.0.0.1:3030/mainnet/transaction/broadcast").send_json(&transaction) {
        Ok(id) => {
            // Remove the quotes from the response.
            let response_string = id.into_string()?.trim_matches('\"').to_string();
            ensure!( response_string == transaction_id.to_string(), "The response does not match the transaction id. ({response_string} != {transaction_id})");
            println!( "⌛ Execution {transaction_id} has been broadcast to {}.", DEFAULT_ENDPOINT);
            Ok(response_string)
        },
        Err(error) => {
            let error_message = match error {
                ureq::Error::Status(code, response) => {
                    format!("(status code {code}: {:?})", response.into_string()?)
                }
                ureq::Error::Transport(err) => format!("({err})"),
            };
            bail!( "❌ Failed to broadcast execution to {}: {}", DEFAULT_ENDPOINT, error_message)
        }
    };
}
#[macro_export]
macro_rules! generate_bindings {
    ($program_name:ident, {
        [$({$function_name:ident, ($($input_name:ident : $input_type:ty),*), ($($output_type:ty),*)},)*],
        [{$($record_name:ident, ($($record_field:ident : $record_field_type:ty),*) );*}],
    }) => {
        use leology::bindings::*;
        #[derive(Debug)]
        $(pub struct $record_name {
            pub record: Record<Nw, Plaintext<Nw>>,
        }
        impl ToValue<Nw> for $record_name {
            fn to_value(&self) -> Value<Nw> {
                Value::Record(self.record.clone())
            }
        }
        impl FromValue<Nw> for $record_name {
            fn from_value(value: Value<Nw>) -> Self {
                match value {
                    Value::Record(record) => {
                        Self{record}
                    },
                    _ => panic!(),
                }
            }
        }
        impl $record_name {
            pub fn new(record: Record<Nw, Plaintext<Nw>>) -> Self {
                $record_name { record }
            }
            $(pub fn $record_field(&self) -> $record_field_type {
                let entry = self.record.data()
                    .get(&Identifier::try_from(stringify!($record_field)).unwrap()).unwrap();
                let value = entry.to_value();
                <$record_field_type>::from_value(value)
            })*
        })*
        pub struct $program_name {
            pub package: Package<Nw>,
        }
        impl $program_name {
            pub fn new(vm: &mut VM<Nw, ConsensusMemory<Nw>>, deployer: &Account<Nw>) -> Result<Self> {
                let query = Query::from(DEFAULT_ENDPOINT);
                let package = Package::open("build".as_ref())?;
                let deployment: Deployment<Nw> = package.deploy::<AleoV0>(None)
                    .expect("Error in package.deploy.");
                let deployment_id = deployment.to_deployment_id()?;

                let transaction = {
                    let rng = &mut rand::thread_rng();
                    let (minimum_deployment_cost, (_, _, _)) = deployment_cost(&deployment)?;
                    let fee_authorization = vm.authorize_fee_public(
                        deployer.private_key(),
                        minimum_deployment_cost,
                        0,
                        deployment_id,
                        rng,
                        )?;
                    let fee = vm.execute_fee_authorization(fee_authorization, Some(query), rng)?;
                    let owner = ProgramOwner::new(deployer.private_key(), deployment_id, rng)?;

                    Transaction::from_deployment(owner, deployment, fee)?
                };
                println("Result of boroadcast deployment: {}", broadcast_transaction(transaction)?);
                println!("✅ Created deployment transaction for '{}'", deployment_id.to_string());
                Ok(Self { package })
            }
            $(
            pub fn $function_name(&self,
                                  vm: &VM<Nw, ConsensusMemory<Nw>>,
                                  account: &Account<Nw>,
                                  $($input_name: $input_type),*) -> Result<($($output_type),*), Error> {
                let args: Vec<Value<Nw>> = vec![
                    $(($input_name).to_value()),*
                ];
                println!("Transaction of function {}:", stringify!($function_name));
                // Execute transaction with package
                let rng = &mut rand::thread_rng();
                let function_name = Identifier::try_from(stringify!($function_name)).expect("Could not make identifier.");
                println!("Executing with package.");
                let (response, execution, metrics) =
                    self.package.execute::<AleoV0, _>(
                        DEFAULT_ENDPOINT.parse().unwrap(), 
                        account.private_key(),
                        function_name,
                        &args,
                        rng).expect("Execution error");
                let mut outputs_iter = response.outputs().into_iter();
                // Broadcast transaction
                println!("Executing with vm.");
                //let program_name_str = format!("{}.aleo", stringify!($program_name));
                let program_name_str = "dev.aleo";
                let program_id = ProgramID::from_str(&program_name_str).unwrap();
//                let function_id = FunctionID::from_str(stringify!($program_name)).unwrap();
                let transaction = vm.execute(
                        account.private_key(),
                        (program_id, stringify!($function_name).to_string()),
                        args.iter(),
                        None,
                        0,
                        Some(Query::from(DEFAULT_ENDPOINT)),
                        rng,
                        )?;
                println!("Response from transaction broadcast: {}", broadcast_transaction(transaction)?);
                Ok(($(
                    <$output_type>::from_value(outputs_iter.next().unwrap().clone())
                ),*))


            }
            )*
        }
    };
}

