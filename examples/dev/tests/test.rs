#[cfg(test)]
mod tests {
    use leology::*;

    include!("dev.rs");

    #[test]
    fn devtest() {
        //let credits: Program<Nw> = Program::credits().unwrap();
        let mut vm = initialize_vm().unwrap();
        // Privately mint 100 tokens for Bob.
        let alice = new_account(Some("0".to_string())).unwrap();
        let token = Dev::new(&mut vm, &alice).unwrap();
        let record = token.create_record(&mut vm, &alice, alice.address(), 10u64).unwrap();
        println!("{:?}", record.owner());
        println!("{:?}", record.number());
        println!("{:#?}", record);
        assert_eq!(record.number(), 10u64);
        let new_record = token.consume_record(&mut vm, &alice, record).unwrap();
        dbg!(new_record);
    }
}
