use crate::*;
pub use aleo_std::StorageMode;
pub use indexmap::IndexMap;
pub use rand;
pub use snarkvm::circuit::AleoV0;
pub use snarkvm::console::program::*;
pub use snarkvm::ledger::query::*;
pub use snarkvm::ledger::store::helpers::memory::ConsensusMemory;
pub use snarkvm::ledger::store::ConsensusStorage;
pub use snarkvm::ledger::store::ConsensusStore;
pub use std::path::{Path, PathBuf};
pub use ureq;

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
            Entry::Public(entry) | Entry::Private(entry) | Entry::Constant(entry) => {
                Value::Plaintext(entry.clone())
            }
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
    let response = ureq::get("http://127.0.0.1:3030/testnet3/program/dev.aleo").call();

    // Deserialize the program.
    match response {
        Ok(response) => response.into_json().map_err(|err| err.into()),
        Err(err) => match err {
            ureq::Error::Status(_status, response) => {
                bail!(response
                    .into_string()
                    .unwrap_or("Response too large!".to_owned()))
            }
            err => bail!(err),
        },
    }
}
/// Fetch the public balance in microcredits associated with the address from the given endpoint.
pub fn get_public_balance(address: &Address<Nw>, endpoint: &str) -> Result<u64> {
    // Initialize the program id and account identifier.
    let credits = ProgramID::<Nw>::from_str("credits.aleo")?;
    let account_mapping = Identifier::<Nw>::from_str("account")?;

    // Send a request to the query node.
    let response =
        ureq::get(&format!("{endpoint}/testnet3/program/{credits}/mapping/{account_mapping}/{address}")).call();

    // Deserialize the balance.
    let balance: Result<Option<Value<Nw>>> = match response {
        Ok(response) => response.into_json().map_err(|err| err.into()),
        Err(err) => match err {
            ureq::Error::Status(_status, response) => {
                bail!(response.into_string().unwrap_or("Response too large!".to_owned()))
            }
            err => bail!(err),
        },
    };

    // Return the balance in microcredits.
    match balance {
        Ok(Some(Value::Plaintext(Plaintext::Literal(Literal::<Nw>::U64(amount), _)))) => Ok(*amount),
        Ok(None) => Ok(0),
        Ok(Some(..)) => bail!("Failed to deserialize balance for {address}"),
        Err(err) => bail!("Failed to fetch balance for {address}: {err}"),
    }
}

pub fn initialize_vm() -> Result<VM<Nw, ConsensusMemory<Nw>>> {
    let store = ConsensusStore::<Nw, ConsensusMemory<Nw>>::open(Some(3))?;
    let vm = VM::from(store)?;
    Ok(vm)
}
pub fn broadcast_transaction(transaction: Transaction<Nw>) -> Result<String> {
    let transaction_id = transaction.id();
    ensure!(!transaction.is_fee(), "The transaction is a fee transaction and cannot be broadcast");
    // Send the deployment request to the local development node.
    match ureq::post("http://127.0.0.1:3030/testnet3/transaction/broadcast").send_json(&transaction) {
        Ok(id) => {
            dbg!(&id);
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
            bail!("❌ Failed to broadcast execution to {}: {}", DEFAULT_ENDPOINT, error_message)
        }
    }
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
            pub fn new(deployer: &Account<Nw>) -> Result<Self> {
                let query = Query::from(DEFAULT_ENDPOINT);
                let package = Package::open("build".as_ref())?;
                let deployment: Deployment<Nw> = package.deploy::<AleoV0>(None)
                    .expect("Error in package.deploy.");
                let deployment_id = deployment.to_deployment_id()?;

                let transaction = {
                    let rng = &mut rand::thread_rng();
                    let store = ConsensusStore::<Nw, ConsensusMemory<Nw>>::open(StorageMode::Production)?;
                    let vm = VM::from(store)?;
                    let (minimum_deployment_cost, (_, _)) = deployment_cost(&deployment)?;
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
                println!("Result of boroadcast deployment: {}", broadcast_transaction(transaction)?);
                println!("✅ Created deployment transaction for '{}'", deployment_id.to_string());
                Ok(Self { package })
            }
            $(
            pub fn $function_name(&self,
                                  account: &Account<Nw>,
                                  $($input_name: $input_type),*) -> Result<($($output_type),*), Error> {
                let program_name = stringify!($program_name).to_string();
                let function_name = stringify!($function_name).to_string();
                let args: Vec<Value<Nw>> = vec![
                    $(($input_name).to_value()),*
                ];
                println!("Transaction of function {}:", stringify!($function_name));
                // Execute transaction with package
                let query = "http://127.0.0.1:3030".to_string();
                let private_key = account.private_key();
                let priority_fee = 0;
                //let locator = Locator::<Nw>::from_str(&format!("{}/{}", &program_name, &function_name))?;
                let locator = Locator::<Nw>::new(ProgramID::try_from("dev.aleo")?, Identifier::from_str(&function_name)?);
                let transaction = {
                    let rng = &mut rand::thread_rng();
                    let storage_mode = StorageMode::Production;
                    let store = ConsensusStore::<Nw, ConsensusMemory<Nw>>::open(storage_mode)?;
                    let vm = VM::from(store)?;
                    let program_id = ProgramID::try_from("dev.aleo")?;
                    let function_id = Identifier::from_str(&function_name).unwrap();
                    load_program(&query, &mut vm.process().write(), &program_id)?;
                    let fee_record = None;
                    vm.execute(
                        &private_key,
                        (program_id, function_id),
                        args.iter(),
                        fee_record,
                        priority_fee,
                        Some(Query::from(&query)),
                        rng,)?
                };
                let public_balance = get_public_balance(&account.address(), &query)?;
                let storage_cost = transaction
                    .execution()
                    .ok_or_else(|| anyhow!("The transaction does not contain an execution"))?
                    .size_in_bytes()?;
                let base_fee = storage_cost.saturating_add(priority_fee);
                if public_balance < base_fee {
                    bail!(
                        "❌ The public balance of {} is insufficient to pay the base fee for `{}`",
                        public_balance,
                        locator.to_string()
                        );
                }
                println!("✅ Created execution transaction for '{}'", locator.to_string());
                println!("Response from transaction broadcast: {}", broadcast_transaction(transaction)?);

                println!("Executing with package to get outputs.");
                let rng = &mut rand::thread_rng();
                let (response, execution, metrics) =
                    self.package.execute::<AleoV0, _>(
                        DEFAULT_ENDPOINT.parse().unwrap(),
                        account.private_key(),
                        Identifier::try_from(function_name)?,
                        &args,
                        rng).expect("Execution error");
                let mut outputs_iter = response.outputs().into_iter();
                

                Ok(($(
                    <$output_type>::from_value(outputs_iter.next().unwrap().clone())
                ),*))
            }
            )*
        }
    };
}
