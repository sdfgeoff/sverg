use super::depgraph::DepGraph;
use std::collections::HashSet;

use super::{Operation, OperationStage, LocatedOperation};
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub enum OrderCalculationError {
    /// One of the operations depended on a node that is not present in the depgraph
    /// or for some other reason a lookup for a node in the graph failed.
    UnknownDependency,

    /// Was unable to complete execution of the graph for the given memory size
    ResourceLimitExceeded,

    /// Something wrong in the implementation (should not be triggerable by
    /// external data - even invalid external data). Hopefully should not occur
    InternalError(String),

    /// No operations can be performed, but not all operations in the graph are included in
    /// the stages
    UnexecutedOperations,

    /// Did not manage to compute execution order within a finite amount of operations.
    /// This may indicate recursion in the input graph....
    IterationLimitReached
}


pub fn compute_execution<I: std::fmt::Debug + std::hash::Hash + Eq + Clone>(
    graph: &DepGraph<I>,
    output_nodes: Vec<I>,
    memory_size: usize,
) -> Result<Vec<OperationStage<I>>, OrderCalculationError> {
    let mut stages = Vec::new();

    let mut prev_memory_state: Vec<Option<I>> = vec![None; memory_size];
    let mut del_after: Vec<LocatedOperation<I>> = Vec::new();

    for (addr, output_node) in output_nodes.iter().enumerate() {
        prev_memory_state[addr] = Some(output_node.clone());
        del_after.push(LocatedOperation {
            id: output_node.clone(), 
            addr
        });
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
        delete_after: del_after,
    };
    stages.push(tmp_first_stage.clone());

    let mut remaining_ops: HashSet<I> = HashSet::from_iter(graph.iter_nodes().cloned());

    // We know that if each execution stage runs one operation, then we should not need to loop
    // more than the number of operations to run. 
    // This gives us a hard end criteria to prevent infinite loops. Something has gone wrong if we reach the end of this
    // without a break
    let mut did_break_from_loop = false;
    for _ in 0..remaining_ops.len() + 1 {
        let prev_stage = &stages.last().ok_or(OrderCalculationError::InternalError("Stages is empty!".to_string()))?;
        let prev_alloc_before_ids: Vec<I> = prev_stage
            .allocate_before
            .iter()
            .map(|op| op.id.clone())
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

        // Op, Dependencies, address
        let mut candidate_ops: Vec<(I, Vec<I>, usize)> = vec![];
        
        for (index, op) in new_memory_state.iter().enumerate() {
            if let Some(op) = op {
                let deps = graph.depends_on(&op).ok_or(OrderCalculationError::UnknownDependency)?;
                candidate_ops.push((op.clone(), deps.clone(), index));
            }
        }

        let all_candidate_operations: Vec<I> = candidate_ops.iter().map(|x| x.0.clone()).collect();
        for remaining_op in &remaining_ops {
            // TODO: Likely slow
            let remaining_op_depends_on = graph.depends_on(&remaining_op).ok_or(OrderCalculationError::UnknownDependency)?;
            for rem in remaining_op_depends_on.iter() {
                if all_candidate_operations.contains(rem) {
                    candidate_ops.retain(|o| &o.0 != rem);
                }
            }
        }

        if candidate_ops.len() == 0 {
            if all_candidate_operations.len() != 0 {
                return Err(OrderCalculationError::InternalError("Unable to find candidate operation, but there are still some remaining".to_string()))
            }
            if candidate_ops.len() != 0 {
                return Err(OrderCalculationError::UnexecutedOperations)
            }
            did_break_from_loop = true;
            break;
        }

        // Sort the candidate operations so we pick the best one according to some heuristics
        candidate_ops.sort_unstable_by(|a, b| {
            let a_deps = a.1.len();
            let b_deps = b.1.len();
            return a_deps.cmp(&b_deps);
        });

        let (operation, operation_depends, operation_addr) = candidate_ops
            .first()
            .ok_or(OrderCalculationError::InternalError("No candidate operations at time to pick one".to_string()))?
            .clone();

        remaining_ops.remove(&operation);
        let mut allocate_before: Vec<LocatedOperation<I>> = Vec::new();
        let mut delete_after: Vec<LocatedOperation<I>> = Vec::new();

        for dep in operation_depends.iter() {
            if !graph.contains(dep) {
                return Err(OrderCalculationError::UnknownDependency);
            } 
            let some_dep = Some(dep.clone());

            if !new_memory_state.contains(&some_dep) {
                let available_slot = new_memory_state
                    .iter()
                    .position(|x| x == &None)
                    .ok_or(OrderCalculationError::ResourceLimitExceeded)?; // # ValueError = Out of memory
                new_memory_state[available_slot] = some_dep; // Cannot fail as the slot id s determined above
                delete_after.push(LocatedOperation{
                    id: dep.clone(), 
                    addr: available_slot
                });
            }
        }

        allocate_before.push(LocatedOperation{
            id: operation.clone(), 
            addr: operation_addr
        });

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


    if !did_break_from_loop {
        return Err(OrderCalculationError::IterationLimitReached);
    }

    stages.reverse();
    if stages.pop() != Some(tmp_first_stage) {
        return Err(OrderCalculationError::InternalError("First stage should have been temporary".to_string()));
    }

    Ok(stages)
}


#[cfg(test)]
fn position<I: Clone + Eq + std::fmt::Debug>(order: &Vec<OperationStage<I>>, operation: I) -> usize {
    return order.iter().position(|x| x.operation.0.id == operation).expect("Operation not in execution stages!");
}


#[test]
fn test_chain_ordering() {
    let mut graph = DepGraph::default();
    // 1 -> 2 -> 3
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);

    let order = compute_execution(&graph, vec![1], 10).expect("Computation Failed");

    assert!(order.len() == 3);
    assert!(position(&order, 1) > position(&order, 2));
    assert!(position(&order, 2) > position(&order, 3));
}

#[test]
fn test_wide_ordering() {
    let mut graph = DepGraph::default();
    graph.insert(1, vec![2, 3, 4]);
    graph.insert(2, vec![]);
    graph.insert(3, vec![]);
    graph.insert(4, vec![]);

    let order = compute_execution(&graph, vec![1], 10).expect("Computation Failed");

    assert!(order.len() == 4);
    assert!(position(&order, 1) > position(&order, 2));
    assert!(position(&order, 1) > position(&order, 3));
    assert!(position(&order, 1) > position(&order, 4));
}


#[test]
fn test_unknown_dep_1() {
    let mut graph = DepGraph::default();
    graph.insert(1, vec![2]);

    let order = compute_execution(&graph, vec![1], 10);
 
    assert_eq!(
        order.unwrap_err(),
        OrderCalculationError::UnknownDependency
    );
}

#[test]
fn test_unknown_dep_2() {
    let mut graph = DepGraph::default();
    graph.insert(1, vec![]);

    let order: Result<_, OrderCalculationError> = compute_execution(&graph, vec![2], 10);

    assert_eq!(
        order.unwrap_err(),
        OrderCalculationError::UnknownDependency
    );
}

#[test]
fn test_unexecuted() {
    todo!("Write remaining tests for the compute_execution function")
}
