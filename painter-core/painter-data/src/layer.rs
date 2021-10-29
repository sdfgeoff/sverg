use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::id_map::OperationId;


#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Layer {
    pub name: String,
    pub blend_operation_id: Option<OperationId>,
}
