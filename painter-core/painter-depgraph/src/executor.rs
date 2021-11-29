use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Operation<I: Clone> {
    pub id: I,
    pub depends_on: Vec<I>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OperationStage<I: Clone> {
    pub operation: (Operation<I>, usize),
    pub allocate_before: Vec<(I, usize)>,
    pub delete_after: Vec<(I, usize)>,
}

#[derive(Debug, PartialEq)]
pub enum ExecutorError {
    /// Specified memory location already contains something!
    MemoryOverwrite,

    /// Freeing memory when there is nothing there
    MemoryFreeingEmpty,

    /// Attempting to free an operation that has never been allocated
    MemoryFreeingUnallocated,

    /// Removing an operation from memory before it was executed!
    MemoryFreeingUnexecuted,

    /// The internal map of where things are stored does not agree with where the ExecutionStage thinks things are stored.
    MemoryMapError,

    /// The internal map disagrees with what is known to be stored at that location. This is an implementation error in the
    /// executor and no stages, valid or invalid should trigger this error. Errors of this class could be avoided by a different
    /// implementation (aka there is internal data redundancy), but may result in from lower performance or more complex code.
    MemoryMapErrorInternal,

    /// Tried to allocate/access outside the number of allowed bounds (specified at invocation time)
    MemoryHitResourceLimit,

    /// Operation was allocated already during this execution cycle and has been re-allocated.
    /// In fact, it may currently be allocated!
    OperationReallocated,

    /// Current operation has not been assigned a place in memory to write to!
    OperationNotAllocated,

    /// Operation was run previously
    OperationRunTwice,

    /// Dependency of the current operation is not allocated
    DependencyNotAllocated,

    /// Dependency of the current operation has not yet executed
    DependencyNotExecuted,
}

fn fmt_memory<I: Debug>(memory: &Vec<(Option<I>, bool)>) -> String {
    let mut out_str = String::new();

    for item in memory {
        if let Some(id) = &item.0 {
            out_str += &format!(
                "| {:?} {} ",
                id,
                match item.1 {
                    true => "☑",
                    false => "☐",
                }
            )
        } else {
            out_str += "|       "
        }
    }
    out_str += "|";
    out_str
}

/// This executor explicitly errors if anything incorrect is detected, and prints the allocated resources at each stage.
/// You specify the operations it should execute and the number of registers available to the machine.
///
/// No work is actually performed, but it may be useful to inspect the operation of this executor and to ensure any array of stages is valid
pub fn debug_executor<I: Debug + std::cmp::PartialEq + std::hash::Hash + Clone + Eq>(
    stages: Vec<OperationStage<I>>,
    register_count: usize,
) -> Result<(), ExecutorError> {
    let mut memory: Vec<(Option<I>, bool)> = Vec::with_capacity(register_count);

    for _r in 0..register_count {
        memory.push((None, false));
    }

    let mut memory_map = std::collections::HashMap::new();

    println!("    {}", fmt_memory(&memory));

    for stage in stages.iter() {
        // Allocate Space
        for (op, addr) in stage.allocate_before.iter() {
            {
                let existing = memory
                    .get(*addr)
                    .ok_or(ExecutorError::MemoryHitResourceLimit)?;
                if existing != &(None, false) {
                    return Err(ExecutorError::MemoryOverwrite);
                };
                if memory_map.contains_key(op) {
                    return Err(ExecutorError::OperationReallocated);
                }
            }
            *memory
                .get_mut(*addr)
                .ok_or(ExecutorError::MemoryHitResourceLimit)? = (Some(op.clone()), false);
            memory_map.insert(op.clone(), *addr);
        }

        println!("    {}", fmt_memory(&memory));

        let (current_operation, current_operation_addr) = &stage.operation;

        // Perform Operation
        {
            // Ensure dependencies for current operation exist
            for dep in current_operation.depends_on.iter() {
                let dep_addr = memory_map
                    .get(dep)
                    .ok_or(ExecutorError::DependencyNotAllocated)?;
                let (dep_cur_allocated, dep_already_executed) = &memory[*dep_addr];
                if dep_cur_allocated.as_ref() != Some(dep) {
                    // The ID stored in the memory map and the ID allocated in memory differ. Since both of these are internal to
                    // this function this is very very bad
                    return Err(ExecutorError::MemoryMapErrorInternal);
                }
                if !dep_already_executed {
                    return Err(ExecutorError::DependencyNotExecuted);
                }
            }

            // Validation
            let addr = memory_map
                .get(&current_operation.id)
                .ok_or(ExecutorError::OperationNotAllocated)?;
            let (cur_allocated, already_executed) = &memory
                .get(*current_operation_addr)
                .ok_or(ExecutorError::MemoryHitResourceLimit)?;

            if addr != current_operation_addr {
                return Err(ExecutorError::MemoryMapError);
            }
            if cur_allocated.as_ref() != Some(&current_operation.id) {
                // The ID stored in the memory map and the ID allocated in memory differ. Since both of these are internal to
                // this function this is very very bad
                return Err(ExecutorError::MemoryMapErrorInternal);
            };
            if *already_executed {
                return Err(ExecutorError::OperationRunTwice);
            }
        }
        memory
            .get_mut(*current_operation_addr)
            .ok_or(ExecutorError::MemoryHitResourceLimit)?
            .1 = true;
        println!("{:?} {}", current_operation.id, fmt_memory(&memory));

        // Remove Old
        for (del_op, del_op_addr) in stage.delete_after.iter() {
            {
                if !memory_map.contains_key(&del_op) {
                    return Err(ExecutorError::MemoryFreeingUnallocated);
                }
                let (del_cur_allocated, del_already_executed) = &memory
                    .get(*del_op_addr)
                    .ok_or(ExecutorError::MemoryHitResourceLimit)?;

                if del_cur_allocated == &None {
                    return Err(ExecutorError::MemoryFreeingEmpty);
                }
                if del_cur_allocated.as_ref() != Some(del_op) {
                    return Err(ExecutorError::MemoryMapError);
                }
                if !del_already_executed {
                    return Err(ExecutorError::MemoryFreeingUnexecuted);
                }
            }

            *memory
                .get_mut(*del_op_addr)
                .ok_or(ExecutorError::MemoryHitResourceLimit)? = (None, false)
        }

        println!("    {}", fmt_memory(&memory));
    }
    Ok(())
}

#[test]
fn test_executor_memory_overwrite_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };

    let stages = vec![OperationStage {
        operation: (a, 0),
        allocate_before: vec![('A', 0), ('B', 0)],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryOverwrite)
    );
}

#[test]
fn test_executor_reallocate_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a, 0),
        allocate_before: vec![('A', 0), ('A', 1)],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::OperationReallocated)
    );
}

#[test]
fn test_executor_dependency_unallocated_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec!['B'],
    };
    let stages = vec![OperationStage {
        operation: (a, 0),
        allocate_before: vec![('A', 0)],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::DependencyNotAllocated)
    );
}

#[test]
fn test_executor_dependency_not_executed_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec!['B'],
    };
    let stages = vec![OperationStage {
        operation: (a, 0),
        allocate_before: vec![('A', 0), ('B', 1)],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::DependencyNotExecuted)
    );
}

#[test]
fn test_executor_opertion_not_allocated_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a, 0),
        allocate_before: vec![],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::OperationNotAllocated)
    );
}

#[test]
fn test_executor_opertion_run_twice_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![
        OperationStage {
            operation: (a.clone(), 0),
            allocate_before: vec![('A', 0)],
            delete_after: vec![],
        },
        OperationStage {
            operation: (a, 0),
            allocate_before: vec![],
            delete_after: vec![],
        },
    ];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::OperationRunTwice)
    );
}

#[test]
fn test_executor_freeing_empty_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 0),
        allocate_before: vec![('A', 0)],
        delete_after: vec![('A', 1)],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryFreeingEmpty)
    );
}

#[test]
fn test_executor_freeing_unallocated_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 0),
        allocate_before: vec![('A', 0)],
        delete_after: vec![('B', 0)],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryFreeingUnallocated)
    );
}

#[test]
fn test_executor_freeing_unexecuted_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 0),
        allocate_before: vec![('A', 0), ('B', 1)],
        delete_after: vec![('B', 1)],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryFreeingUnexecuted)
    );
}

#[test]
fn test_executor_mem_map_error1_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 0),
        allocate_before: vec![('A', 1)],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryMapError)
    );
}

#[test]
fn test_executor_mem_map_error2_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 0),
        allocate_before: vec![('A', 0), ('B', 1)],
        delete_after: vec![('B', 0)],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryMapError)
    );
}

#[test]
fn test_executor_memory_limit_allocate_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 0),
        allocate_before: vec![('A', 0), ('B', 20)],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryHitResourceLimit)
    );
}

#[test]
fn test_executor_memory_limit_operate_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 20),
        allocate_before: vec![('A', 0)],
        delete_after: vec![],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryHitResourceLimit)
    );
}

#[test]
fn test_executor_memory_limit_delete_detect() {
    let a = Operation {
        id: 'A',
        depends_on: vec![],
    };
    let stages = vec![OperationStage {
        operation: (a.clone(), 0),
        allocate_before: vec![('A', 0)],
        delete_after: vec![('A', 20)],
    }];
    assert_eq!(
        debug_executor(stages, 10),
        Err(ExecutorError::MemoryHitResourceLimit)
    );
}
