program dev.aleo;

record rec:
    owner as address.private;
    number as u64.private;


mapping mapping_number:
	key as address.public;
	value as u64.public;

function create_record:
    input r0 as address.private;
    input r1 as u64.private;
    cast r0 r1 into r2 as rec.record;
    async create_record r0 r1 into r3;
    output r2 as rec.record;
    output r3 as dev.aleo/create_record.future;

finalize create_record:
    input r0 as address.public;
    input r1 as u64.public;
    set r1 into mapping_number[r0];


function consume_record:
    input r0 as rec.record;
    async consume_record r0.owner into r1;
    output r1 as dev.aleo/consume_record.future;

finalize consume_record:
    input r0 as address.public;
    set 100u64 into mapping_number[r0];
