#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use leology::core::*;

    include!("token.rs");

    lazy_static!{}

    #[test]
    fn private_minting_should_work() {
        // Privately mint 100 tokens for Bob.
        let alice = new_account(None).unwrap();
        let bob = new_account(None).unwrap();
        let token = Token::try_load().unwrap();
        let record = token.mint_private(&alice, bob.address(), 100u64).unwrap();
        println!("{:?}", record.amount());
        println!("{:#?}", record);
        assert_eq!(record.amount(), 100u64);
    }
    #[test]
    fn private_to_private_transfer_should_work() {
        let alice = new_account(None).unwrap();
        let bob = new_account(None).unwrap();
        let token = Token::try_load().unwrap();
        let record = token.mint_private(&alice, bob.address(), 100u64).unwrap();
        assert_eq!(record.amount(), 100u64);
        let (remaining, transferred) = token.transfer_private(&bob, record, alice.address(), 20u64).unwrap();
        println!("{:?}", remaining.amount());
        println!("{:?}", transferred.amount());
        assert_eq!(remaining.amount(), 80u64);
        assert_eq!(transferred.amount(), 20u64)
    }
/*
    #[test]
    fn public_minting_should_work() {
        // Publicly mint 100 tokens for Alice.
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("mint_public", vec![&ALICE_ADDRESS, "100u64"]).unwrap(),
                &alice.private_key(),
            )
            .expect("Could not mint 100 tokens for Alice");
        println!("{:?}", tx);
        println!("{:?}", res);
    }

    #[test]
    fn public_to_public_transfer_should_work() {
        // Publicly transfer 10 tokens from Alice to Bob
        let (res, tx) = ENGINE
            .execute(
                FunctionDef::try_from("transfer_public", vec![&BOB_ADDRESS, "10u64"]).unwrap(),
                &ALICE_PK,
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
                FunctionDef::try_from("transfer_public_to_private", vec![&BOB_ADDRESS, "30u64"])
                    .unwrap(),
                &ALICE_PK,
            )
            .expect("Could not perform the public-to-private transfer from Alice to Bob");
        println!("{:?}", tx);
        println!("{:?}", res);
    }
*/
}
