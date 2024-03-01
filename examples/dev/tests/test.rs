#[cfg(test)]
mod tests {
    use leology::*;

    include!("dev.rs");

    #[test]
    fn devtest() {
        //let credits: Program<Nw> = Program::credits().unwrap();
        // Privately mint 100 tokens for Bob.
        //let alice = new_account(Some("0".to_string())).unwrap();
        let alice = Account::try_from("APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs").unwrap();
        let dev = Dev::new(&alice).unwrap();
        let record = dev.create_record(&alice, alice.address(), 10u64).unwrap();
        println!("{:?}", record.owner());
        println!("{:?}", record.number());
        println!("{:#?}", record);
        assert_eq!(record.number(), 10u64);
        let new_record = dev.consume_record(&alice, record).unwrap();
        dbg!(new_record);
    }
}
