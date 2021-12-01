use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DepGraph<I: Hash + Eq + Debug + Clone> {
    pub nodes: HashMap<I, Vec<I>>,
}

impl<I: Hash + Eq + Debug + Clone> Default for DepGraph<I> {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}

impl<I: Hash + Eq + Debug + Clone> DepGraph<I> {
    /// Inserts an operation and a list of items it requires in order to run.
    /// Eg to execute the operation, all the items in the vector must have been run in advance.
    pub fn insert(&mut self, operation: I, depends_on: Vec<I>) {
        self.nodes.insert(operation, depends_on);
    }

    /// Returns a vector describing what operations must be run for the supplied operation
    /// to be able to be executed.
    pub fn depends_on(&self, operation: &I) -> Option<&Vec<I>> {
        self.nodes.get(operation)
    }

    /// Returns a vector describing what operations depend upon the supplied operation.
    /// Ie after executing the suppied operation, the operations in the returned vector
    /// can then be executed.
    pub fn dependees(&self, operation: &I) -> Vec<&I> {
        self.nodes
            .iter()
            .filter_map(|(k, v)| {
                if v.contains(&operation) {
                    Some(k)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Generates a string representation of the depgraph that can be visualized using
    /// dot https://en.wikipedia.org/wiki/DOT_(graph_description_language)
    /// This is useful for debugging
    /// It requires a "formatter" function. This is distinct from the DEBUG or DISPLAY
    /// features so that you can use it to do a lookup in some external resource.
    pub fn generate_dotgraph(&self, formatter: &dyn Fn(&I) -> String) -> String {
        let mut outstr = "digraph depsgraph {\n".to_string();
        for (node, depends_on) in self.nodes.iter() {
            let mut node_hasher = DefaultHasher::new();
            node.hash(&mut node_hasher);
            let node_hash = node_hasher.finish();
            outstr += &format!("    op_{} [label={:?}];\n", node_hash, formatter(node));

            for dependant_node in depends_on {
                let mut dep_hasher = DefaultHasher::new();
                dependant_node.hash(&mut dep_hasher);
                let dep_hash = dep_hasher.finish();
                outstr += &format!("    op_{} -> op_{};\n", node_hash, dep_hash);
            }
        }
        outstr += "}\n";
        outstr
    }

    pub fn contains(&self, node: &I) -> bool {
        self.nodes.contains_key(node)
    }


    /// Inserts a new operation into the tree "on top" of where the old one was.
    /// Eg:
    /// ```ignore
    ///    A -----> Base ---------> C
    /// ```
    /// Goes to:
    /// ```ignore
    ///    A ----> New Operation -----> Base  ----> C
    /// ```
    ///
    ///
    /// If there are multiple dependants and dependees they are moved as follows:
    ///
    /// ```ignore
    ///              B
    ///              |
    ///              V
    /// A ------->  Base --------> D
    ///              |
    ///              V
    ///              C
    /// ```
    /// Turns into
    ///
    /// ```ignore
    ///           B
    ///           |
    ///           V
    /// A ---->  New ---------> Base --------> D
    ///                          |
    ///                          V
    ///                          C
    /// ```
    pub fn operate_on(&mut self, new_operation: I, base: I) {
        let dependees:Vec<I> = self.dependees(&base).iter().map(|x| (*x).clone()).collect();
        let dependants: Vec<I> = self.depends_on(&base).expect("Base not in depsgraph").to_vec();
        assert!(
            self.nodes.insert(new_operation.clone(), dependants).is_none(),
            "operate_on assumes completely new entry"
        );
        self.nodes.insert(new_operation.clone(), vec![base.clone()]);

        for depe in dependees {
            let existing_children = self.nodes.get_mut(&depe).expect("");
            let index = existing_children
                .iter()
                .position(|x| *x == base)
                .expect("result from get_parents did not contain this parent");
            existing_children[index] = new_operation.clone();
        }
    }
}

#[test]
fn test_depends_on() {
    let mut d = DepGraph::default();

    d.insert(1, vec![2, 3]);
    d.insert(2, vec![3]);

    assert_eq!(d.depends_on(&1), Some(&vec![2, 3]));
    assert_eq!(d.depends_on(&2), Some(&vec![3]));
}

#[test]
fn test_dependees() {
    let mut d = DepGraph::default();

    d.insert(1, vec![2, 3]);
    d.insert(2, vec![3]);

    assert!(d.dependees(&2).contains(&&1));
    assert!(d.dependees(&3).contains(&&2));
    assert!(d.dependees(&3).contains(&&1));
}

#[test]
fn test_dotfile() {
    let mut d = DepGraph::default();

    d.insert(1, vec![2, 3]);
    d.insert(2, vec![3]);

    println!("{}", d.generate_dotgraph(&|x| format!("{:?}", x)))
}



#[test]
fn test_operate_on_simple() {
    let mut d = DepGraph::default();

    let a = "A".to_string();
    let base = "Base".to_string();
    let c = "C".to_string();
    let new = "New".to_string();

    d.insert(a.clone(), vec![base.clone()]);
    d.insert(base.clone(), vec![c.clone()]);
    d.insert(c.clone(), vec![]);

    // println!("{}", d.generate_dotgraph(&|x| format!("{}", x)));

    d.operate_on(new.clone(), base.clone());

    // println!("{}", d.generate_dotgraph(&|x| format!("{}", x)));

    assert!(d.depends_on(&a).expect("Missing dep").contains(&new));
    assert!(d.depends_on(&new).expect("Missing dep").contains(&base));
    assert!(d.depends_on(&base).expect("Missing dep").contains(&c));
}

#[test]
fn test_operate_on_with_multi_deps() {

    let mut graph = DepGraph::default();

    let a = "A";
    let b = "B";
    let base = "Base";
    let c = "C";
    let d = "D";
    let new = "New";

    graph.insert(base.clone(), vec![c.clone(), d.clone()]);
    graph.insert(a.clone(), vec![base.clone()]);
    graph.insert(b.clone(), vec![base.clone()]);
    graph.insert(c.clone(), vec![]);
    graph.insert(d.clone(), vec![]);

    // println!("{}", graph.generate_dotgraph(&|x| format!("{}", x)));

    graph.operate_on(new.clone(), base.clone());

    // println!("{}", graph.generate_dotgraph(&|x| format!("{}", x)));

    assert!(graph.depends_on(&base).expect("Missing Dep").contains(&c));
    assert!(graph.depends_on(&base).expect("Missing Dep").contains(&d));
    assert!(graph.depends_on(&new).expect("Missing Dep").contains(&base));
    assert!(graph.depends_on(&a).expect("Missing Dep").contains(&new));
    assert!(graph.depends_on(&b).expect("Missing Dep").contains(&new));

    // Parents no longer reference the original base
    assert!(!graph.depends_on(&a).expect("Missing Dep").contains(&base));
    assert!(!graph.depends_on(&b).expect("Missing Dep").contains(&base));
}
