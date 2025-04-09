// The 'dev' program.
program dev.aleo {
    mapping mapping_number: address => u64;
    record rec {
        owner: address,
        number: u64,
    }
    async transition create_record(to: address, number: u64) -> (rec, Future) {
        return (rec { owner: to, number: number }, finalize_create_record(to, number));
    }
    async function finalize_create_record(to: address, number: u64) {
        Mapping::set(mapping_number, to, number);
    }
    async transition consume_record(r: rec) -> Future {
        return finalize_consume_record(r.owner);
    }
    async function finalize_consume_record(owner: address) {
        Mapping::set(mapping_number, owner, 100u64);
    }
}
