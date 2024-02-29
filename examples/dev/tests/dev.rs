leology::generate_bindings! {
    Dev, {
        [
        { create_record, (to: Address<Nw>, number: u64), (Rec) },
        { consume_record, (record: Rec), () },
        ],
        [{ Rec, (owner: Address<Nw>, number: u64) }],
    }
}
