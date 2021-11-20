use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::color_primitives::Color;
use crate::depgraph::DepGraph;
use crate::id_map::{BrushIdMap, LayerIdMap, OperationIdMap};

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MetaData {
    #[pyo3(get, set)]
    pub preview_canvas_size: [u32; 2],

    #[pyo3(get, set)]
    pub canvas_background_color: Color,
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    #[pyo3(get)]
    pub brushes: BrushIdMap,

    #[pyo3(get)]
    pub operations: OperationIdMap,

    pub depgraph: DepGraph,

    #[pyo3(get)]
    pub layers: LayerIdMap,

    #[pyo3(get)]
    pub metadata: MetaData,
}
