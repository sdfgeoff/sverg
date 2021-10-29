use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::id_map::AddIncr;
use crate::operation::OperationId;

#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LayerId(u64);

impl LayerId {
    pub fn new() -> Self {
        LayerId(0)
    }
}

impl AddIncr for LayerId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
}

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Layer {
    pub name: String,
    pub blend_operation_id: Option<OperationId>,
}
