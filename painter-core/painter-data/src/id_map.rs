//! A hashmap-like object that auto-creates unique keys.
//! There is a lot of duplicate code in here which is BAD
//! It can all be solved with type generics, but type generics
//! aren't supported by pyo3's #[pyclass] macro, and for now it isn't worth
//! the effort to write our own pyclass-ification system or procedural macros

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::brush::Brush;
use super::layer::Layer;
use super::operation::Operation;

/// Keys of the IdMap must implement this trait as it is
/// used to find a new unique key.
pub trait AddIncr {
    fn increment(&mut self) -> Self;
}

/// Making 1000 strokes per second we will run out of ID's
/// in about 585 million years. I think that's enough
#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BrushId(u64);

#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LayerId(u64);

#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OperationId(u64);

#[pyclass]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BrushIdMap {
    map: HashMap<BrushId, Brush>,
    id: BrushId,
}
#[pyclass]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LayerIdMap {
    map: HashMap<LayerId, Layer>,
    id: LayerId,
}
#[pyclass]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OperationIdMap {
    map: HashMap<OperationId, Operation>,
    id: OperationId,
}

// Duplicate code starts below

impl Default for BrushId {
    fn default() -> Self {
        BrushId(0)
    }
}
impl Default for LayerId {
    fn default() -> Self {
        LayerId(0)
    }
}
impl Default for OperationId {
    fn default() -> Self {
        OperationId(0)
    }
}

impl AddIncr for BrushId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
}
impl AddIncr for LayerId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
}
impl AddIncr for OperationId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
}

impl Default for BrushIdMap {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            id: BrushId::default(),
        }
    }
}
impl Default for LayerIdMap {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            id: LayerId::default(),
        }
    }
}
impl Default for OperationIdMap {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            id: OperationId::default(),
        }
    }
}

pub trait IdMapBase {
    type Index;
    type Value;

    /// Inserts an item into the map and returns it's ID
    fn insert(&mut self, item: Self::Value) -> Self::Index;
    fn get_mut(&mut self, key: &Self::Index) -> Option<&mut Self::Value>;
    fn get_mut_unchecked(&mut self, key: &Self::Index) -> &mut Self::Value;
}

impl IdMapBase for BrushIdMap {
    type Index = BrushId;
    type Value = Brush;
    fn insert(&mut self, item: Brush) -> BrushId {
        let uniq = self.id.increment();
        assert!(
            self.map.insert(uniq.clone(), item).is_none(),
            "IDMap Detected Dupicate Key"
        );
        uniq
    }

    fn get_mut(&mut self, key: &BrushId) -> Option<&mut Brush> {
        self.map.get_mut(key)
    }

    fn get_mut_unchecked(&mut self, key: &BrushId) -> &mut Brush {
        self.map.get_mut(key).expect("IDMap Get Unckecked Failed")
    }
}

impl IdMapBase for LayerIdMap {
    type Index = LayerId;
    type Value = Layer;

    fn insert(&mut self, item: Layer) -> LayerId {
        let uniq = self.id.increment();
        assert!(
            self.map.insert(uniq.clone(), item).is_none(),
            "IDMap Detected Dupicate Key"
        );
        uniq
    }

    fn get_mut(&mut self, key: &LayerId) -> Option<&mut Layer> {
        self.map.get_mut(key)
    }

    fn get_mut_unchecked(&mut self, key: &LayerId) -> &mut Layer {
        self.map.get_mut(key).expect("IDMap Get Unckecked Failed")
    }
}

impl IdMapBase for OperationIdMap {
    type Index = OperationId;
    type Value = Operation;
    fn insert(&mut self, item: Operation) -> OperationId {
        let uniq = self.id.increment();
        assert!(
            self.map.insert(uniq.clone(), item).is_none(),
            "IDMap Detected Dupicate Key"
        );
        uniq
    }

    fn get_mut(&mut self, key: &OperationId) -> Option<&mut Operation> {
        self.map.get_mut(key)
    }

    fn get_mut_unchecked(&mut self, key: &OperationId) -> &mut Operation {
        self.map.get_mut(key).expect("IDMap Get Unckecked Failed")
    }
}

#[pymethods]
impl BrushIdMap {
    fn list_ids(&self) -> Vec<BrushId> {
        self.map.keys().cloned().collect()
    }
}

#[pymethods]
impl LayerIdMap {
    fn list_ids(&self) -> Vec<LayerId> {
        self.map.keys().cloned().collect()
    }
}

#[pymethods]
impl OperationIdMap {
    fn list_ids(&self) -> Vec<OperationId> {
        self.map.keys().cloned().collect()
    }
}
