use painter_depgraph::{default_executor, LocatedOperation, Operation, OperationStage};

fn to_op<I: Clone + std::fmt::Debug>(op: I, addr: usize) -> LocatedOperation<I> {
    LocatedOperation { id: op, addr: addr }
}

fn main() {
    let a = Operation {
        id: 'A',
        depends_on: vec!['B', 'G'],
    };
    let b = Operation {
        id: 'B',
        depends_on: vec!['C'],
    };
    let c = Operation {
        id: 'C',
        depends_on: vec!['D', 'F'],
    };
    let d = Operation {
        id: 'D',
        depends_on: vec!['E'],
    };
    let e = Operation {
        id: 'E',
        depends_on: vec![],
    };
    let f = Operation {
        id: 'F',
        depends_on: vec!['E'],
    };
    let g = Operation {
        id: 'G',
        depends_on: vec!['F'],
    };

    let stages = vec![
        OperationStage {
            operation: (e, 0),
            allocate_before: vec![to_op('E', 0)],
            delete_after: vec![],
        },
        OperationStage {
            operation: (f, 3),
            allocate_before: vec![to_op('F', 3)],
            delete_after: vec![],
        },
        OperationStage {
            operation: (g, 2),
            allocate_before: vec![to_op('G', 2)],
            delete_after: vec![],
        },
        OperationStage {
            operation: (d, 1),
            allocate_before: vec![to_op('D', 1)],
            delete_after: vec![to_op('E', 0)],
        },
        OperationStage {
            operation: (c, 0),
            allocate_before: vec![to_op('C', 0)],
            delete_after: vec![to_op('D', 1), to_op('F', 3)],
        },
        OperationStage {
            operation: (b, 1),
            allocate_before: vec![to_op('B', 1)],
            delete_after: vec![to_op('C', 0)],
        },
        OperationStage {
            operation: (a, 0),
            allocate_before: vec![to_op('A', 0)],
            delete_after: vec![to_op('B', 1), to_op('G', 2)],
        },
    ];

    default_executor(
        stages,
        10,
        &mut |op| println!("Loading {:?} into {:?}", op.id, op.addr),
        &mut |op| println!("Deleting {:?} from {:?}", op.id, op.addr),
        &mut |op, deps| println!("Executing {:?} ({:?}) deps: {:?}", op.id, op.addr, deps),
    )
    .expect("Arrgh!");
}
