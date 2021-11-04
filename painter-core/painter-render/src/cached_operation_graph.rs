
/// Computes what operations need to be performed based on changes from a previous
/// set of operations and depgraph.
/// 
/// Currently this is a partial implementation testing out some ideas. It is not fully
/// implemented and needs more thought.
struct CachedOperationGraph {
    operations: OperationIdMap,
    depgraph: DepGraph,
    cache: HashMap<OperationId, u8>,
}

impl Default for CachedOperationGraph {
    fn default() -> Self {
        Self {
            operations: OperationIdMap::default(),
            depgraph: DepGraph::default(),
            cache: HashMap::new()
        }
    }
}

impl CachedOperationGraph {
    // Outputs the operations that need to be recomputed
    fn compute_changed(&self, new_graph: &DepGraph, new_operations: &OperationIdMap) -> Vec<OperationId>{
        // let output_node = get_output_node(&new_operations).expect("No Output Node");
        
        let mut changed: Vec<OperationId> = Vec::new();

        let dirty_operation_ids: Vec<OperationId> = new_operations.iter().filter_map(|(k, v)| {
            if Some(v) != self.operations.get(k) {
                Some(*k)
            } else {
                None
            }
        }).collect();
        changed.extend(&dirty_operation_ids);

        // Follow parents util find cached content
        let flipped_graph = new_graph.flip(); // Allows us to work upstream more efficiently by inverting the grpah all at once
        let mut dirty_roots = Vec::new();
        for dirty_op in dirty_operation_ids.iter() {
            let mut tips = vec![*dirty_op];
            while let Some(tip) = tips.pop() {
                //todo!("This will hang with dependency cycles. Possibly implement a get_children_recursive_until_stop_criteria function");
                if !self.cache.contains_key(&tip) {
                    dirty_roots.push(tip);
                    tips.extend(flipped_graph.get_children(tip));
                }
            }
        }
        changed.extend(dirty_roots);

        // Now we need to walk the other direction
        let mut dirty_tips = Vec::new();
        for dirty_op in dirty_operation_ids.iter() {
            dirty_tips.extend(new_graph.get_children_recursive_breadth_first(*dirty_op));
        }
        println!("Tips: {:?}", dirty_tips);
        println!("flipped_graph: {:?}", flipped_graph);
        changed.extend(dirty_tips);

        println!("{:?}", changed);

        changed

    }
    fn do_operation(&mut self, id: OperationId, operation: Operation, result: u8) {
        if self.operations.get(&id) == None {
            self.operations.force(id, operation);
        } else {
            self.operations.alter(id, operation);
        }
        self.cache.insert(id, result);
    }
}

#[test]
fn test_cached_operation_graph_picks_up_new_nodes() {
    let mut cached_graph = CachedOperationGraph::default();
    let mut operations = OperationIdMap::default();
    let mut dep_graph = DepGraph::default();

    let op_a = Operation::Tag("TestA".to_string());
    let id_a = operations.insert(op_a.clone());

    assert!(cached_graph.compute_changed(&dep_graph, &operations).contains(&id_a));
    cached_graph.do_operation(id_a, op_a, 8);

    let b = operations.insert(Operation::Tag("TestB".to_string()));
    assert!(!cached_graph.compute_changed(&dep_graph, &operations).contains(&id_a));
    assert!(cached_graph.compute_changed(&dep_graph, &operations).contains(&b));
}


#[test]
fn test_cached_operation_graph_dirties_child_nodes() {
    let mut cached_graph = CachedOperationGraph::default();
    let mut operations = OperationIdMap::default();
    let mut dep_graph = DepGraph::default();

    let op_a = Operation::Tag("TestA".to_string());
    let id_a = operations.insert(op_a.clone());

    let op_b = Operation::Tag("TestB".to_string());
    let id_b = operations.insert(op_b.clone());

    dep_graph.insert_as_child(id_b, id_a);

    cached_graph.do_operation(id_a, op_a, 8);
    cached_graph.do_operation(id_b, op_b, 7);

    assert!(cached_graph.compute_changed(&dep_graph, &operations).len() == 0);
    
    operations.force(id_a, Operation::Tag("TestA2".to_string()));

    println!("{:?}", cached_graph.compute_changed(&dep_graph, &operations));
    assert!(cached_graph.compute_changed(&dep_graph, &operations).contains(&id_a));
    assert!(cached_graph.compute_changed(&dep_graph, &operations).contains(&id_b));
}
