#[cfg(test)]
mod tests {
    use leology::engine::{Engine, FunctionDef, Identifier};

    lazy_static! {
        static ref engine: &'static Engine = Engine::try_load()
            .expect("Failure while reading the contract code. Did you compile your program?");
    }

    const alice_pk: &str = "APrivateKey1zkp1w8PTxrRgGfAtfKUSq43iQyVbdQHfhGbiNPEg2LVSEXR";
    const alice_address: &str = "aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q";
    const bob_pk: &str = "APrivateKey1zkpFo72g7N9iFt3JzzeG8CqsS5doAiXyFvNCgk2oHvjRCzF";
    const bob_address: &str = "aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z";

    #[test]
    fn public_minting_should_work() {
        // Publicly mint 100 tokens for Alice.
        engine
            .execute(
                FunctionDef::try_from("mint_public", vec![alice_address, 100u64]),
                alice_pk,
            )
            .expect("Could not mint 100 tokens for Alice");
    }

    #[test]
    fn private_minting_should_work() {
        // Privately mint 100 tokens for Bob.
        engine
            .execute(
                FunctionDef::try_from("mint_private", vec![bob_address, 100u64]),
                alice_pk,
            )
            .expect("Could not mint 100 tokens for Alice");
    }
}
