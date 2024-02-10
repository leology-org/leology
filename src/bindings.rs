use snarkvm::prelude::*;
pub use rand;

pub trait ToValue<N: Network> {
    fn to_value(&self) -> Value<N>;
}
impl ToValue<Testnet3> for u64 {
    fn to_value(&self) -> Value<Testnet3> {
        Value::from(Literal::U64(U64::new(*self)))
    }
}
impl ToValue<Testnet3> for Address<Testnet3> {
    fn to_value(&self) -> Value<Testnet3> {
        Value::from(Literal::Address(*self))
    }
}
impl ToValue<Testnet3> for Entry<Testnet3, Plaintext<Testnet3>> {
    fn to_value(&self) -> Value<Testnet3> {
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
impl FromValue<Testnet3> for Record<Testnet3, Plaintext<Testnet3>> {
    fn from_value(value: Value<Testnet3>) -> Self {
        match value {
            Value::Record(value) => value,
            _ => panic!("Wrong type."),
        }
    }
}
impl FromValue<Testnet3> for u64 {
    fn from_value(value: Value<Testnet3>) -> Self {
        match value {
            Value::Plaintext(Plaintext::Literal(Literal::U64(v), _)) => *v,
            _ => panic!("Wrong type."),
        }
    }
}
#[macro_export]
macro_rules! generate_bindings {
    ($program_name:ident, {
        [$({$function_name:ident, ($($input_name:ident : $input_type:ty),*), ($($output_type:ty),*)},)*],
        [{$($record_name:ident, ($($record_field:ident : $record_field_type:ty),*) );*}],
    }) => {
        use leology::bindings::{ToValue, FromValue};
        use leology::bindings::rand;
        const SNARKVM_CONTRACTS_BUILD_FOLDER: &str = "build";
        const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:3030";
        pub struct $program_name {
            package: Package<Testnet3>,
        }
        #[derive(Debug)]
        $(pub struct $record_name {
            pub record: Record<Testnet3, Plaintext<Testnet3>>,
        }
        impl ToValue<Testnet3> for $record_name {
            fn to_value(&self) -> Value<Testnet3> {
                Value::Record(self.record.clone())
            }
        }
        impl FromValue<Testnet3> for $record_name {
            fn from_value(value: Value<Testnet3>) -> Self {
                match value {
                    Value::Record(record) => {
                        Self{record}
                    },
                    _ => panic!(),
                }
            }
        }
        impl $record_name {
            pub fn new(record: Record<Testnet3, Plaintext<Testnet3>>) -> Self {
                $record_name { record }
            }
            $(pub fn $record_field(&self) -> $record_field_type {
                let entry = self.record.data()
                    .get(&Identifier::try_from(stringify!($record_field)).unwrap()).unwrap();
                let value = entry.to_value();
                u64::from_value(value)
            })*
        })*
        impl $program_name {
            pub fn try_load() -> Result<Self> {
                // load the package from the ./contracts folder
                let full_path = format!("{}",SNARKVM_CONTRACTS_BUILD_FOLDER);
                let package = Package::open(full_path.as_ref())?;

                Ok(Self { package })
            }
            $(
            pub fn $function_name(&self,
                                  account: &Account<Testnet3>,
                                  $($input_name: $input_type),*) -> Result<($($output_type),*), Error> {
                let rng = &mut rand::thread_rng();
                let args: Vec<Value<Testnet3>> = vec![
                    $(($input_name).to_value()),*
                ];
                let (response, execution, _) = self.package.execute::<Aleo, _>(
                    DEFAULT_ENDPOINT.parse().unwrap(),
                    account.private_key(),
                    Identifier::from_str(stringify!($function_name)).unwrap(),
                    &args,
                    rng,
                )?;
                let transaction = Transaction::from_execution(execution, None)?;

                let mut outputs_iter = response.outputs().into_iter();
                Ok(($(
                    <$output_type>::from_value(outputs_iter.next().unwrap().clone())
                ),*))
            }
            )*
        }
    };
}

