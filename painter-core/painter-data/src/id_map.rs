//! A hashmap-like object that auto-creates unique keys.
//! There is a lot of duplicate code in here which is BAD
//! It can all be solved with type generics, but type generics
//! aren't supported by pyo3's #[pyclass] macro, and for now it isn't worth
//! the effort to write our own pyclass-ification system or procedural macros

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::brush::{Brush, Glyph};
use super::layer::Layer;
use super::operation::Operation;



/// Keys of the IdMap must implement this trait as it is
/// used to find a new unique key.
pub trait IncrId {
    fn increment(&mut self) -> Self;
    fn val(&self) -> u64;
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

/// Making 1000 strokes per second we will run out of ID's
/// in about 585 million years. I think that's enough
#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BrushId(u64);

#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GlyphId(u64);

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
pub struct GlyphIdMap {
    map: HashMap<GlyphId, Glyph>,
    id: GlyphId,
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

macro_rules! impl_id_struct {
    ($id_type:ty) => {

        impl Default for $id_type {
            fn default() -> Self {
                Self(0)
            }
        }

        impl IncrId for $id_type {
            fn increment(&mut self) -> Self {
                let out = Self(self.0);
                self.0 += 1;
                out
            }
            fn val(&self) -> u64 {
                self.0
            }
        }
    }
}



macro_rules! impl_id_map_struct {
    ($map_type: ty,$id_type:ty, $value_type: ty) => {

        impl Default for $map_type {
            fn default() -> Self {
                Self {
                    map: HashMap::new(),
                    id: <$id_type>::default(),
                }
            }
        }

        impl IdMapBase for $map_type {
            type Index = $id_type;
            type Value = $value_type;
            fn insert(&mut self, item: Self::Value) -> Self::Index {
                let uniq = self.id.increment();
                assert!(
                    self.map.insert(uniq.clone(), item).is_none(),
                    "IDMap Detected Dupicate Key"
                );
                uniq
            }
            fn alter(&mut self, id: Self::Index, item: Self::Value) {
                assert!(
                    self.map.contains_key(&id),
                    "Attempting to alter non-existant ID"
                );
                self.map.insert(id, item);
            }
            fn force(&mut self, id: Self::Index, item: Self::Value) {
                self.map.insert(id, item);
            }
            fn get(&self, key: &Self::Index) -> Option<&Self::Value> {
                self.map.get(key)
            }
            fn get_unchecked(&self, key: &Self::Index) -> &Self::Value {
                self.map.get(key).expect("IDMap Get Unckecked Failed")
            }
            fn get_mut(&mut self, key: &Self::Index) -> Option<&mut Self::Value> {
                self.map.get_mut(key)
            }
            fn get_mut_unchecked(&mut self, key: &Self::Index) -> &mut Self::Value {
                self.map.get_mut(key).expect("IDMap Get Unckecked Failed")
            }
            fn iter(&self) -> std::collections::hash_map::Iter<Self::Index, Self::Value> {
                self.map.iter()
            }
        }

        #[pymethods]
        impl $map_type {
            fn list_ids(&self) -> Vec<$id_type> {
                self.map.keys().cloned().collect()
            }
        }

    }
}



impl_id_struct!(BrushId);
impl_id_struct!(GlyphId);
impl_id_struct!(LayerId);
impl_id_struct!(OperationId);


impl_id_map_struct!(BrushIdMap, BrushId, Brush);
impl_id_map_struct!(GlyphIdMap, GlyphId, Glyph);
impl_id_map_struct!(LayerIdMap, LayerId, Layer);
impl_id_map_struct!(OperationIdMap, OperationId, Operation);
