#[cfg(test)]
mod tests {
    use leology::*;

    include!("dev.rs");

    #[test]
    fn devtest() {
        //let credits: Program<Nw> = Program::credits().unwrap();
        // Privately mint 100 tokens for Bob.
        //let alice = new_account(Some("0".to_string())).unwrap();
        let alice =
            Account::try_from("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
                .unwrap();
        let dev = Dev::new(&alice).unwrap();
        let (record, future) = dev.create_record(&alice, alice.address(), 10u64).unwrap();
        println!("{:?}", record.number());
        println!("{:#?}", record);
        assert_eq!(record.number(), 10u64);
        let future = dev.consume_record(&alice, record).unwrap();
        dbg!(future);
    }
}
