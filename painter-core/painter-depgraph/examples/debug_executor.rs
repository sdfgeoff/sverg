use painter_depgraph::{debug_executor, Operation, OperationStage};

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
            allocate_before: vec![('E', 0)],
            delete_after: vec![],
        },
        OperationStage {
            operation: (f, 3),
            allocate_before: vec![('F', 3)],
            delete_after: vec![],
        },
        OperationStage {
            operation: (g, 2),
            allocate_before: vec![('G', 2)],
            delete_after: vec![],
        },
        OperationStage {
            operation: (d, 1),
            allocate_before: vec![('D', 1)],
            delete_after: vec![('E', 0)],
        },
        OperationStage {
            operation: (c, 0),
            allocate_before: vec![('C', 0)],
            delete_after: vec![('D', 1), ('F', 3)],
        },
        OperationStage {
            operation: (b, 1),
            allocate_before: vec![('B', 1)],
            delete_after: vec![('C', 0)],
        },
        OperationStage {
            operation: (a, 0),
            allocate_before: vec![('A', 0)],
            delete_after: vec![('B', 1), ('G', 2)],
        },
    ];

    debug_executor(stages, 10).expect("Arrgh!");
}
