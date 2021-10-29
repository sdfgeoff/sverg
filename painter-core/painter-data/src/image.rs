use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::color_primitives::Color;
use crate::id_map::{BrushIdMap, LayerIdMap, OperationId, OperationIdMap};

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MetaData {
    #[pyo3(get, set)]
    pub preview_canvas_size: [i32; 2],

    #[pyo3(get, set)]
    pub canvas_background_color: Color,
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    #[pyo3(get)]
    pub file_format_version: (i32, i32, i32),

    #[pyo3(get)]
    pub brushes: BrushIdMap,

    #[pyo3(get)]
    pub operations: OperationIdMap,

    pub depgraph: HashMap<OperationId, Vec<OperationId>>,

    #[pyo3(get)]
    pub layers: LayerIdMap,

    #[pyo3(get)]
    pub metadata: MetaData,
}
