use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use pyo3::prelude::*;

use crate::brush::{Brush, BrushId};
use crate::color_primitives::Color;
use crate::id_map::IdMap;
use crate::layer::{Layer, LayerId};
use crate::operation::{Operation, OperationId};

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

    pub brushes: IdMap<BrushId, Brush>,
    pub operations: IdMap<OperationId, Operation>,

    pub depgraph: HashMap<OperationId, Vec<OperationId>>,
    pub layers: IdMap<LayerId, Layer>,

    #[pyo3(get, set)]
    pub metadata: MetaData,
}
