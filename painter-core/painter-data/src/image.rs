use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::brush::{Brush, BrushId};
use crate::color_primitives::Color;
use crate::id_map::IdMap;
use crate::layer::{Layer, LayerId};
use crate::operation::{Operation, OperationId};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MetaData {
    pub preview_canvas_size: [i32; 2],
    pub canvas_background_color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub file_format_version: (i32, i32, i32),
    pub brushes: IdMap<BrushId, Brush>,
    pub operations: IdMap<OperationId, Operation>,

    pub depgraph: HashMap<OperationId, Vec<OperationId>>,
    pub layers: IdMap<LayerId, Layer>,
    pub metadata: MetaData,
}

impl Default for  Image {
    fn default() -> Self {
        Self {
            file_format_version: (0, 0, 1),
            brushes: IdMap::new(BrushId::new()),
            operations: IdMap::new(OperationId::new()),
            depgraph: HashMap::new(),
            layers: IdMap::new(LayerId::new()),
            metadata: MetaData {
                preview_canvas_size: [1920, 1080],
                canvas_background_color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            },
        }
    }
}
