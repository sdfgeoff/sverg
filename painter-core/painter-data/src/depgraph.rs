use std::collections::HashMap;
use crate::id_map::OperationId;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct DepGraph {
    children: HashMap<OperationId, Vec<OperationId>>
}

impl Default for DepGraph {
    fn default() -> Self {
        Self {
            children: HashMap::new()
        }
    }
}
impl DepGraph {
    /// Inserts a node as a child of another node
    /// ```
    /// use painter_data::depgraph::DepGraph;
    /// use painter_data::id_map::{OperationId, IncrId};
    /// let mut d = DepGraph::default();
    /// let mut id_store = OperationId::default();
    /// let id1 = id_store.increment();
    /// let id2 = id_store.increment();
    /// let id3 = id_store.increment();
    /// 
    /// d.insert_as_child(id1, id2);
    /// d.insert_as_child(id2, id3);
    /// 
    /// assert!(d.get_children(id2).contains(&id1));
    /// assert!(d.get_children(id3).contains(&id2));
    /// ```
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

    /// ```
    /// use painter_data::depgraph::DepGraph;
    /// use painter_data::id_map::{OperationId, IncrId};
    /// let mut d = DepGraph::default();
    /// let mut id_store = OperationId::default();
    /// let id1 = id_store.increment();
    /// let id2 = id_store.increment();
    /// let id3 = id_store.increment();
    /// 
    /// d.insert_as_child(id1, id2);
    /// d.insert_as_child(id2, id3);
    /// 
    /// assert!(d.get_parents(id2).contains(&id3));
    /// assert!(d.get_parents(id1).contains(&id2));
    /// ```
    pub fn get_parents(&self, child: OperationId) -> Vec<OperationId> {
        let parents = self.children.iter().filter_map(|(k, v)| {
            if v.contains(&child) {
                Some(k)
            } else {
                None
            }
        });
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
    /// ```
    /// use painter_data::depgraph::DepGraph;
    /// use painter_data::id_map::{OperationId, IncrId};
    /// let mut d = DepGraph::default();
    /// let mut id_store = OperationId::default();
    /// let base_child = id_store.increment();
    /// let base = id_store.increment();
    /// let base_parent = id_store.increment();
    /// let new = id_store.increment();
    /// 
    /// d.insert_as_child(base_child, base);
    /// d.insert_as_child(base, base_parent);
    /// 
    /// d.operate_on(new, base);
    /// 
    /// assert!(d.get_children(base).contains(&base_child));
    /// assert!(d.get_children(new).contains(&base));
    /// assert!(d.get_children(base_parent).contains(&new));
    /// ```
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
    /// use painter_data::depgraph::DepGraph;
    /// use painter_data::id_map::{OperationId, IncrId};
    /// let mut d = DepGraph::default();
    /// let mut id_store = OperationId::default();
    /// let base_child1 = id_store.increment();
    /// let base_child2 = id_store.increment();
    /// let base = id_store.increment();
    /// let base_parent1 = id_store.increment();
    /// let base_parent2 = id_store.increment();
    /// let new = id_store.increment();
    /// 
    /// d.insert_as_child(base_child1, base);
    /// d.insert_as_child(base_child2, base);
    /// d.insert_as_child(base, base_parent1);
    /// d.insert_as_child(base, base_parent2);
    /// 
    /// d.operate_on(new, base);
    /// 
    /// assert!(d.get_children(base).contains(&base_child1));
    /// assert!(d.get_children(base).contains(&base_child2));
    /// assert!(d.get_children(new).contains(&base));
    /// assert!(d.get_children(base_parent1).contains(&new));
    /// assert!(d.get_children(base_parent2).contains(&new));
    /// 
    /// // Parents no longer reference the original base
    /// assert!(!d.get_children(base_parent1).contains(&base));
    /// assert!(!d.get_children(base_parent2).contains(&base));
    /// ```
    pub fn operate_on(&mut self, new_operation: OperationId, base: OperationId) {
        let parents = self.get_parents(base);
        assert!(self.children.insert(new_operation, vec![base]).is_none(), "operate_on assumes completely new entry");
        for parent in parents {
            let existing_children = self.get_children_mut(parent);
            let index = existing_children.iter().position(|x| *x == base).expect("result from get_parents did not contain this parent");
            existing_children[index] = new_operation;
        }
    }
}