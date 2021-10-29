use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::id_map::AddIncr;
use crate::stroke::StrokeData;

/// Making 1000 strokes per second we will run out of ID's
/// in about 585 million years. I think that's enough
#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OperationId(u64);

impl OperationId {
    pub fn new() -> Self {
        OperationId(0)
    }
}

impl AddIncr for OperationId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Stroke(StrokeData),
    BlendLayers(),
}
