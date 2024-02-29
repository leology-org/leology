// The 'dev' program.
program dev.aleo {
    mapping mapping_number: address => u64;
    record rec {
        owner: address,
        number: u64,
    }
    transition create_record(to: address, number: u64) -> rec {
        return rec { owner: to, number: number } then finalize(to, number);
    }
    finalize create_record(to: address, number: u64) {
        Mapping::set(mapping_number, to, number);
    }
    transition consume_record(r: rec) {
        return then finalize(r.owner);
    }
    finalize consume_record(owner: address) {
        Mapping::set(mapping_number, owner, 100u64);
    }
}