use crate::id_map::OperationId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A dependency graph. It would be nice to genericize this,
/// but then you can't derive serialize/deserialize on it.
/// AKA: not worth it at the moment.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct DepGraph {
    children: HashMap<OperationId, Vec<OperationId>>,
}

impl Default for DepGraph {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
        }
    }
}
impl DepGraph {
    pub fn insert_as_child(&mut self, child: OperationId, parent: OperationId) {
        if let Some(existing_children) = self.children.get_mut(&parent) {
            existing_children.push(child);
        } else {
            self.children.insert(parent, vec![child]);
        }
    }

    pub fn get_children(&self, parent: OperationId) -> Vec<OperationId> {
        if let Some(existing_children) = self.children.get(&parent) {
            existing_children.to_vec()
        } else {
            Vec::new()
        }
    }
    pub fn get_children_mut(&mut self, parent: OperationId) -> &mut Vec<OperationId> {
        if !self.children.contains_key(&parent) {
            self.children.insert(parent, Vec::new());
        }
        self.children.get_mut(&parent).unwrap()
    }

    ///Returns the parents of the supplied node. Not as efficient as get_children.
    pub fn get_parents(&self, child: OperationId) -> Vec<OperationId> {
        let parents =
            self.children.iter().filter_map(
                |(k, v)| {
                    if v.contains(&child) {
                        Some(k)
                    } else {
                        None
                    }
                },
            );
        parents.cloned().collect()
    }

    /// Inserts a new operation into the tree "on top" of where the old one was.
    /// Eg:
    /// ```ignore
    ///    BaseChild -----> Base ---------> BaseParent
    /// ```
    /// Goes to:
    /// ```ignore
    ///    BaseChild -----> Base ----> New Operation ----> BaseParent
    /// ```
    ///
    ///
    /// If there are multiple dependants and dependees they are moved as follows:
    ///
    /// ```ignore
    ///                BaseChild2
    ///                    |
    ///                    V
    /// BaseChild1 ---->  Base --------> BaseParent1
    ///                    |
    ///                    V
    ///                 BaseParent2
    /// ```
    /// Turns into
    ///
    /// ```ignore
    ///                BaseChild2
    ///                    |
    ///                    V
    /// BaseChild1 ---->  Base---------> New --------> BaseParent1
    ///                                   |
    ///                                   V
    ///                               BaseParent2
    /// ```
    pub fn operate_on(&mut self, new_operation: OperationId, base: OperationId) {
        let parents = self.get_parents(base);
        assert!(
            self.children.insert(new_operation, vec![base]).is_none(),
            "operate_on assumes completely new entry"
        );
        for parent in parents {
            let existing_children = self.get_children_mut(parent);
            let index = existing_children
                .iter()
                .position(|x| *x == base)
                .expect("result from get_parents did not contain this parent");
            existing_children[index] = new_operation;
        }
    }


    /// Returns all the children of a specific node. 
    /// Works even if dependency cycles are present
    pub fn get_children_recursive_breadth_first(&self, start: OperationId) -> Vec<OperationId> {
        let mut to_search = vec![start];
        let mut all_children = Vec::new();
        let mut searched = HashSet::new();
        while let Some(current_search) = to_search.pop() {
            searched.insert(current_search);
            for child in self.get_children(current_search) {
                all_children.push(child);

                if !searched.contains(&child) && !to_search.contains(&child) {
                    to_search.push(child);
                }
            }
        }
        all_children
    }

    /// Returns a copy of the graph whereby the children become parents and visa-versa
    /// ie: 
    ///    A ----> B ----> C
    /// becomes:
    ///    A <---- B <---- C
    pub fn flip(&self) -> Self {
        let mut parent_graph = DepGraph::default();
        for (parent, children) in self.children.iter() {
            for child in children {
                parent_graph.insert_as_child(*parent, *child);
            }
        }
        parent_graph
    }
}

#[test]
fn test_get_parents() {
    use crate::id_map::{OperationId, IncrId};
    let mut d = DepGraph::default();
    let mut id_store = OperationId::default();
    let id1 = id_store.increment();
    let id2 = id_store.increment();
    let id3 = id_store.increment();
    
    d.insert_as_child(id1, id2);
    d.insert_as_child(id2, id3);
    
    assert!(d.get_parents(id2).contains(&id3));
    assert!(d.get_parents(id1).contains(&id2));
}


#[test]
fn test_insert_as_child() {
    /// Inserts a node as a child of another node
    use crate::id_map::{OperationId, IncrId};

    let mut d = DepGraph::default();
    let mut id_store = OperationId::default();
    let id1 = id_store.increment();
    let id2 = id_store.increment();
    let id3 = id_store.increment();
    
    d.insert_as_child(id1, id2);
    d.insert_as_child(id2, id3);
    
    assert!(d.get_children(id2).contains(&id1));
    assert!(d.get_children(id3).contains(&id2));
    
}

#[test]
fn test_flip() {
    use crate::id_map::{OperationId, IncrId};
    let mut d = DepGraph::default();
    let mut id_store = OperationId::default();
    let a = id_store.increment();
    let b = id_store.increment();
    let c = id_store.increment();
    
    d.insert_as_child(b, a);
    d.insert_as_child(c, b);
    
    assert!(d.get_children(a).contains(&b));
    assert!(d.get_children(b).contains(&c));
    
    let flipped = d.flip();
    assert!(flipped.get_children(b).contains(&a));
    assert!(flipped.get_children(c).contains(&b));
}


#[test]
fn test_get_children_recursive_breadth_first() {
    use crate::id_map::{OperationId, IncrId};

    {
        let mut graph = DepGraph::default();
        let mut id_store = OperationId::default();
        let a = id_store.increment();
        let b = id_store.increment();
        let c = id_store.increment();
        let d = id_store.increment();
        
        // D <---- C <------ B <----- A
        graph.insert_as_child(b, a);
        graph.insert_as_child(c, b);
        graph.insert_as_child(d, c);
        
        let all_children = graph.get_children_recursive_breadth_first(a);
        
        assert_eq!(all_children, vec![b, c, d])
    }

    {
        let mut graph = DepGraph::default();
        let mut id_store = OperationId::default();
        let a = id_store.increment();
        let b = id_store.increment();
        let c = id_store.increment();
        let d = id_store.increment();
        let x = id_store.increment();
        let y = id_store.increment();
        
        // D <---- C <------ B <----- A
        //         X <------/
        //         Y <-----'
        graph.insert_as_child(b, a);
        graph.insert_as_child(c, b);
        graph.insert_as_child(d, c);
        graph.insert_as_child(x, b);
        graph.insert_as_child(y, b);
        
        let all_children = graph.get_children_recursive_breadth_first(a);
        
        assert_eq!(all_children, vec![b, c, x, y, d])
    }
}

#[test]
fn test_get_children_recursive_breadth_first_with_dependency_cycle() {
    use crate::id_map::{OperationId, IncrId};
    
    let mut graph = DepGraph::default();
    let mut id_store = OperationId::default();
    let a = id_store.increment();
    let b = id_store.increment();
    let c = id_store.increment();
    let d = id_store.increment();
    
    graph.insert_as_child(b, a);
    graph.insert_as_child(c, b);
    graph.insert_as_child(d, c);
    graph.insert_as_child(b, c);
    
    let all_children = graph.get_children_recursive_breadth_first(a);
    
    assert!(all_children.contains(&b));
    assert!(all_children.contains(&c));
    assert!(all_children.contains(&d));
    assert!(!all_children.contains(&a));
}

#[test]
fn test_operate_on_simple() {
    use crate::id_map::{OperationId, IncrId};
    let mut d = DepGraph::default();

    let mut id_store = OperationId::default();
    let base_child = id_store.increment();
    let base = id_store.increment();
    let base_parent = id_store.increment();
    let new = id_store.increment();

    d.insert_as_child(base_child, base);
    d.insert_as_child(base, base_parent);

    d.operate_on(new, base);

    assert!(d.get_children(base).contains(&base_child));
    assert!(d.get_children(new).contains(&base));
    assert!(d.get_children(base_parent).contains(&new));

}

#[test]
fn test_operate_on_with_multi_deps() {
    use crate::id_map::{OperationId, IncrId};

    let mut d = DepGraph::default();
    let mut id_store = OperationId::default();

    let base_child1 = id_store.increment();
    let base_child2 = id_store.increment();
    let base = id_store.increment();
    let base_parent1 = id_store.increment();
    let base_parent2 = id_store.increment();
    let new = id_store.increment();
    
    d.insert_as_child(base_child1, base);
    d.insert_as_child(base_child2, base);
    d.insert_as_child(base, base_parent1);
    d.insert_as_child(base, base_parent2);
    
    d.operate_on(new, base);
    
    assert!(d.get_children(base).contains(&base_child1));
    assert!(d.get_children(base).contains(&base_child2));
    assert!(d.get_children(new).contains(&base));
    assert!(d.get_children(base_parent1).contains(&new));
    assert!(d.get_children(base_parent2).contains(&new));
    
    // Parents no longer reference the original base
    assert!(!d.get_children(base_parent1).contains(&base));
    assert!(!d.get_children(base_parent2).contains(&base));
}
