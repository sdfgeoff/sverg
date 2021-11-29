use super::depgraph::DepGraph;
use std::collections::HashSet;

use super::{Operation, OperationStage};
use std::iter::FromIterator;

pub fn compute_execution<I: std::fmt::Debug + std::hash::Hash + Eq + Clone>(
    graph: &DepGraph<I>,
    output_nodes: Vec<I>,
    memory_size: usize,
) -> Result<Vec<OperationStage<I>>, u8> {
    let mut stages = Vec::new();

    let mut prev_memory_state: Vec<Option<I>> = vec![None; memory_size];

    for (addr, output_node) in output_nodes.iter().enumerate() {
        prev_memory_state[addr] = Some(output_node.clone())
    }

    let tmp_first_stage = OperationStage {
        operation: (
            Operation {
                id: output_nodes[0].clone(),
                depends_on: vec![],
            },
            0,
        ),
        allocate_before: vec![],
        delete_after: output_nodes
            .iter()
            .map(|outp| {
                (
                    outp.clone(),
                    prev_memory_state
                        .iter()
                        .position(|x| x.as_ref() == Some(outp))
                        .expect("Mem Consistency Error"),
                )
            })
            .collect(),
    };
    stages.push(tmp_first_stage.clone());

    let mut remaining_ops: HashSet<I> = HashSet::from_iter(graph.nodes.keys().cloned());

    // We will only drain remaining ops if there are no unreachanble entities in the graph, but
    // this gives us a hard end criteria. Something has gone wrong if we reach the end of this
    // without a break
    // TOOO: check if we reached the end without hitting a break
    for _ in 0..remaining_ops.len() {
        let prev_stage = &stages.last().expect("Stages is empty!");
        let prev_alloc_before_ids: Vec<I> = prev_stage
            .allocate_before
            .iter()
            .map(|op| op.0.clone())
            .collect();
        let mut new_memory_state: Vec<Option<I>> = prev_memory_state
            .iter()
            .map(|op| {
                if let Some(op) = op {
                    if prev_alloc_before_ids.contains(op) {
                        None
                    } else {
                        Some(op.clone())
                    }
                } else {
                    None
                }
            })
            .collect();

        let available_ops: Vec<I> = new_memory_state
            .iter()
            .filter_map(|op| op.clone())
            .collect();
        let mut candidate_operations = available_ops.clone();
        for remaining_op in &remaining_ops {
            // TODO: Likely slow
            let remaining_op_depends_on = graph.depends_on(&remaining_op).expect("op not in graph");
            for rem in remaining_op_depends_on.iter() {
                if candidate_operations.contains(rem) {
                    candidate_operations.retain(|o| o != rem);
                }
            }
        }

        if candidate_operations.len() == 0 {
            assert_eq!(
                available_ops.len(),
                0,
                "Unable to find candidate operation, but there are still some remaining"
            );
            assert_eq!(remaining_ops.len(), 0, "Some operations were not executed");
            break;
        }

        //     # Sort the candidate operations so we pick the best one according to some heuristics
        candidate_operations.sort_unstable_by(|a, b| {
            let a_dep = graph
                .depends_on(a)
                .expect("Unknown Dependency Detected")
                .len();
            let b_dep = graph
                .depends_on(b)
                .expect("Unknown Dependency Detected")
                .len();
            return a_dep.cmp(&b_dep);
        });

        let operation: I = candidate_operations
            .first()
            .expect("No Candidate Operations (should have been caught earlier")
            .clone();
        let operation_addr: usize = new_memory_state
            .iter()
            .position(|x| x.as_ref() == Some(&operation))
            .expect("UNable to locate operation (internal error)");
        let operation_depends: Vec<I> = graph
            .depends_on(&operation)
            .expect("Unable to find dep information")
            .clone();

        remaining_ops.remove(&operation);
        let mut allocate_before: Vec<(I, usize)> = Vec::new();
        let mut delete_after: Vec<(I, usize)> = Vec::new();

        for dep in operation_depends.iter() {
            assert!(graph.contains(dep), "Unknown Dependency detected");
            let some_dep = Some(dep.clone());

            if !new_memory_state.contains(&some_dep) {
                let available_slot = new_memory_state
                    .iter()
                    .position(|x| x == &None)
                    .expect("Unable to allocate"); // # ValueError = Out of memory
                new_memory_state[available_slot] = some_dep; // TODO: Failable
                delete_after.push((dep.clone(), available_slot));
            }
        }

        allocate_before.push((operation.clone(), operation_addr));

        let stage = OperationStage {
            operation: (
                Operation {
                    id: operation,
                    depends_on: operation_depends,
                },
                operation_addr,
            ),
            allocate_before,
            delete_after,
        };
        stages.push(stage);

        prev_memory_state = new_memory_state;
    }
    stages.reverse();
    assert_eq!(
        stages.pop(),
        Some(tmp_first_stage),
        "Failed to remove tmp first stage"
    );

    Ok(stages)
}
