use leology::deployment_cost;
use leology::generate_bindings;
use leology::Account;
use leology::Deployment;
use leology::Nw;
use leology::Package;
use leology::Transaction;
use leology::DEFAULT_ENDPOINT;
use leology::VM;

generate_bindings! {
    Dev, {
        [
        { create_record, (to: Address<Nw>, number: u64), (Rec, Future<Nw>) },
        { consume_record, (record: Rec), (Future<Nw>) },
        ],
        [{ Rec, (number: u64) }],
    }
}
