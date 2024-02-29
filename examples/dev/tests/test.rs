#[cfg(test)]
mod tests {
    use leology::*; // Assuming 'leology' is a crate related to Leo program testing.

    use std::time::Instant;
    // Including the development setup script. Ensure 'dev.rs' is well-documented and follows best practices.
    include!("dev.rs");

    /// Tests the `dev` Leo program's functionality.
    ///
    /// This test initializes a virtual machine, creates an account, and performs token operations
    /// to verify the `create_record` and `consume_record` transitions of the `dev` program.
    #[test]
    fn devtest() {
        let start = Instant::now();
        let mut vm = initialize_vm().expect("Failed to initialize VM");
        println!("VM initialized in {:?}", start.elapsed());

        let start = Instant::now();
        let alice = new_account(Some("0".to_string())).expect("Failed to create account");
        println!("Account created in {:?}", start.elapsed());

        let start = Instant::now();
        let token = Dev::new(&mut vm, &alice).unwrap();
        println!("Dev::new completed in {:?}", start.elapsed());

        let start = Instant::now();
        let record = token.create_record(&mut vm, &alice, alice.address(), 10u64).unwrap();
        println!("Record created in {:?}", start.elapsed());

        // Debug prints to observe the record's properties. Consider removing these in the final version.
        println!("Record Owner: {:?}", record.owner());
        println!("Record Number: {:?}", record.number());
        println!("Full Record Details: {:#?}", record);

        // Assert the 'number' field of the record to verify 'create_record' transition.
        assert_eq!(record.number(), 10u64, "Record number does not match expected value");

        // Consume the record and verify the 'consume_record' transition.
        let new_record = token.consume_record(&mut vm, &alice, record).expect("Failed to consume record");

        // Use `dbg!` for debugging; ensure to clean up or comment out for production.
        dbg!(new_record);
    }
}
