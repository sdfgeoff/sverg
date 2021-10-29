//! A hashmap-like object that auto-creates unique keys.
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


/// Keys of the IdMap must implement this trait as it is
/// used to find a new unique key.
pub trait AddIncr {
    fn increment(&mut self) -> Self;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IdMap<I: std::cmp::Eq + std::hash::Hash, T> {
    map: HashMap<I, T>,
    id: I,
}

impl<I: std::cmp::Eq + std::hash::Hash + AddIncr + Clone, T> IdMap<I, T> {
    pub fn new(id: I) -> Self {
        Self {
        map: HashMap::new(),
        id: id,
        }
    }

    // Inserts an item into the map and returns it's ID
    pub fn insert(&mut self, item: T) -> I {
        //self.id = self.id + 1;
        let uniq = self.id.increment();
        assert!(self.map.insert(uniq.clone(), item).is_none(), "IDMap Detected Dupicate Key");
        uniq
    }
}