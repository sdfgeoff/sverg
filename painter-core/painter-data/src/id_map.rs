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
pub trait IncrId {
    fn increment(&mut self) -> Self;
    fn val(&self) -> u64;
}

/// Making 1000 strokes per second we will run out of ID's
/// in about 585 million years. I think that's enough
#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BrushId(u64);

#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LayerId(u64);

#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
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

impl IncrId for BrushId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
    fn val(&self) -> u64 {
        self.0
    }
}
impl IncrId for LayerId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
    fn val(&self) -> u64 {
        self.0
    }
}
impl IncrId for OperationId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
    fn val(&self) -> u64 {
        self.0
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
    fn alter(&mut self, id: Self::Index, item: Self::Value);
    fn force(&mut self, id: Self::Index, item: Self::Value);
    fn get(&self, key: &Self::Index) -> Option<&Self::Value>;
    fn get_unchecked(&self, key: &Self::Index) -> &Self::Value;
    fn get_mut(&mut self, key: &Self::Index) -> Option<&mut Self::Value>;
    fn get_mut_unchecked(&mut self, key: &Self::Index) -> &mut Self::Value;
    fn iter(&self) -> std::collections::hash_map::Iter<Self::Index, Self::Value>;
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
    fn alter(&mut self, id: BrushId, item: Brush) {
        assert!(
            self.map.contains_key(&id),
            "Attempting to alter non-existant ID"
        );
        self.map.insert(id, item);
    }
    fn force(&mut self, id: BrushId, item: Brush) {
        self.map.insert(id, item);
    }
    fn get(&self, key: &BrushId) -> Option<&Brush> {
        self.map.get(key)
    }
    fn get_unchecked(&self, key: &BrushId) -> &Brush {
        self.map.get(key).expect("IDMap Get Unckecked Failed")
    }
    fn get_mut(&mut self, key: &BrushId) -> Option<&mut Brush> {
        self.map.get_mut(key)
    }
    fn get_mut_unchecked(&mut self, key: &BrushId) -> &mut Brush {
        self.map.get_mut(key).expect("IDMap Get Unckecked Failed")
    }
    fn iter(&self) -> std::collections::hash_map::Iter<BrushId, Brush> {
        self.map.iter()
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
    fn alter(&mut self, id: LayerId, item: Layer) {
        assert!(
            self.map.contains_key(&id),
            "Attempting to alter non-existant ID"
        );
        self.map.insert(id, item);
    }
    fn force(&mut self, id: LayerId, item: Layer) {
        self.map.insert(id, item);
    }
    fn get(&self, key: &LayerId) -> Option<&Layer> {
        self.map.get(key)
    }
    fn get_unchecked(&self, key: &LayerId) -> &Layer {
        self.map.get(key).expect("IDMap Get Unckecked Failed")
    }
    fn get_mut(&mut self, key: &LayerId) -> Option<&mut Layer> {
        self.map.get_mut(key)
    }

    fn get_mut_unchecked(&mut self, key: &LayerId) -> &mut Layer {
        self.map.get_mut(key).expect("IDMap Get Unckecked Failed")
    }
    fn iter(&self) -> std::collections::hash_map::Iter<LayerId, Layer> {
        self.map.iter()
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
    fn alter(&mut self, id: OperationId, item: Operation) {
        assert!(
            self.map.contains_key(&id),
            "Attempting to alter non-existant ID"
        );
        self.map.insert(id, item);
    }
    fn force(&mut self, id: OperationId, item: Operation) {
        self.map.insert(id, item);
    }
    fn get(&self, key: &OperationId) -> Option<&Operation> {
        self.map.get(key)
    }
    fn get_unchecked(&self, key: &OperationId) -> &Operation {
        self.map.get(key).expect("IDMap Get Unckecked Failed")
    }

    fn get_mut(&mut self, key: &OperationId) -> Option<&mut Operation> {
        self.map.get_mut(key)
    }

    fn get_mut_unchecked(&mut self, key: &OperationId) -> &mut Operation {
        self.map
            .get_mut(key)
            .expect("IDMap Get Mut Unckecked Failed")
    }
    fn iter(&self) -> std::collections::hash_map::Iter<OperationId, Operation> {
        self.map.iter()
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
