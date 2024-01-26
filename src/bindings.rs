use snarkvm::prelude::*;
pub use rand;

pub trait IntoValue<N: Network> {
    fn into_value(self) -> Value<N>;
}
impl IntoValue<Testnet3> for Address<Testnet3> {
    fn into_value(self) -> Value<Testnet3> {
        Value::from(Literal::Address(self))
    }
}
impl IntoValue<Testnet3> for u64 {
    fn into_value(self) -> Value<Testnet3> {
        Value::from(Literal::U64(U64::new(self)))
    }
}
#[macro_export]
macro_rules! generate_bindings {
    ($program_name:ident, { $($function_name:ident, ($($input_name:ident : $input_type:ty),*), $output_type:ty);* }) => {
        use leology::bindings::IntoValue;
        use leology::bindings::rand;
        const SNARKVM_CONTRACTS_BUILD_FOLDER: &str = "build";
        const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:3030";
        pub struct $program_name {
            package: Package<Testnet3>,
        }
        impl $program_name {
            $(
                pub fn $function_name(&self, account: Account<Testnet3>,$($input_name: $input_type),*) -> $output_type {
                    let rng = &mut rand::thread_rng();
                    let args: Vec<Value<Testnet3>> = vec![
                        $(($input_name).into_value()),*
                    ];
                    let (response, execution, _) = self.package.execute::<Aleo, _>(
                        DEFAULT_ENDPOINT.parse().unwrap(),
                        account.private_key(),
                        Identifier::from_str(stringify!($function_name)).unwrap(),
                        &args,
                        rng,
                    )?;
                    let transaction = Transaction::from_execution(execution, None)?;
                    Ok((response, transaction))
                }
                pub fn try_load() -> Result<Self> {
                    // load the package from the ./contracts folder
                    let full_path = format!("{}",SNARKVM_CONTRACTS_BUILD_FOLDER);
                    let package = Package::open(full_path.as_ref())?;

                    Ok(Self { package })
                }
            )*
        }
    };
}
