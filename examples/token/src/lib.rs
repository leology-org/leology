#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use leology::engine::{Engine, FunctionDef};

    lazy_static! {
        static ref ENGINE: Engine = Engine::try_load()
            .expect("Failure while reading the contract code. Did you compile your program?");
    }

    const ALICE_PK: &str = "APrivateKey1zkp1w8PTxrRgGfAtfKUSq43iQyVbdQHfhGbiNPEg2LVSEXR";
    const ALICE_ADDRESS: &str = "aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q";
    const BOB_PK: &str = "APrivateKey1zkpFo72g7N9iFt3JzzeG8CqsS5doAiXyFvNCgk2oHvjRCzF";
    const BOB_ADDRESS: &str = "aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z";

    #[test]
    fn public_minting_should_work() {
        // Publicly mint 100 tokens for Alice.
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("mint_public", vec![ALICE_ADDRESS, "100u64"]).unwrap(),
                ALICE_PK,
            )
            .expect("Could not mint 100 tokens for Alice");
        println!("{:?}", tx);
        println!("{:?}", res);
    }

    #[test]
    fn private_minting_should_work() {
        // Privately mint 100 tokens for Bob.
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("mint_private", vec![BOB_ADDRESS, "100u64"]).unwrap(),
                BOB_PK,
            )
            .expect("Could not mint 100 tokens for Bob");
        println!("{:?}", tx);
        println!("{:?}", res);
    }

    #[test]
    fn public_to_public_transfer_should_work() {
        // Publicly transfer 10 tokens from Alice to Bob
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("transfer_public", vec![BOB_ADDRESS, "10u64"]).unwrap(),
                ALICE_PK,
            )
            .expect("Could not perform the public-to-public transfer from Alice to Bob");
        println!("{:?}", tx);
        println!("{:?}", res);
    }

    #[test]
    fn public_to_private_transfer_should_work() {
        // Publicly transfer 10 tokens from Alice to Bob
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("transfer_public_to_private", vec![BOB_ADDRESS, "30u64"])
                    .unwrap(),
                ALICE_PK,
            )
            .expect("Could not perform the public-to-private transfer from Alice to Bob");
        println!("{:?}", tx);
        println!("{:?}", res);
    }

    #[test]
    fn private_to_private_transfer_should_work() {
        // Publicly transfer 10 tokens from Alice to Bob

        // we firstly mint again to get the UTXO
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("mint_private", vec![BOB_ADDRESS, "100u64"]).unwrap(),
                BOB_PK,
            )
            .expect("Could not mint 100 tokens for Bob");
        println!("{:?}", tx);
        println!("{:?}", res);
        let output = res.outputs().get(0).unwrap().to_string();
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("transfer_private", vec![&output, ALICE_ADDRESS, "20u64"])
                    .unwrap(),
                BOB_PK,
            )
            .expect("Could not perform the private-to-private transfer from Alice to Bob");
        println!("{:?}", tx);
        println!("{:?}", res);
    }
}
