use pyo3::prelude::*;
use crate::operations::{Operation, OperationId};

use solvent::DepGraph;
use std::collections::HashMap;

#[derive(Clone)]
enum BlendMode {

}

#[derive(Clone)]
struct OperationNode {
    operation_data: Operation,
    blend_mode: BlendMode
}

#[derive(Clone)]
struct Layer {
    tip_operation: OperationId,
    name: String,
}

#[derive(Clone)]
struct CanvasData {
    preview_canvas_size: [i32; 2],
    export_scale: f32,
    background_color: [f32; 4],
}

#[derive(Clone)]
struct MetaData {
    file_format_version: i32,
    unique_operation_id_counter: i128,
}

#[derive(Clone)]
struct Cache {

}

impl Cache {
    pub fn new() -> Self {
        Self {}
    }
}



#[pyclass]
#[derive(Clone)]
pub struct Image {
    depgraph: DepGraph<OperationId>,
    operations: HashMap<OperationId, OperationNode>,
    layers: Vec<Layer>,
    canvas: CanvasData,
    cache: Cache,
    metadata: MetaData,
}


impl Image {
    pub fn new() -> Self {
        let image = Self {
            depgraph: DepGraph::new(),
            operations: HashMap::new(),
            layers: Vec::new(),
            canvas: CanvasData {
                preview_canvas_size: [1920, 1080],
                export_scale: 2.0,
                background_color: [1.0, 1.0, 1.0, 1.0],
            },
            cache: Cache::new(),
            metadata: MetaData {
                file_format_version: 0,
                unique_operation_id_counter: 0,
            }
        }
        image.add_layer("Bakground");

    }

    pub fn get_operation_id(&mut self) -> OperationId {
        self.metadata.unique_operation_id_counter += 1;
        return OperationId(self.metadata.unique_operation_id_counter)
    }

    pub fn add_layer(&mut self, name: String) {
        let op_id = self.get_operation_id()
        self.operations.insert(op_id, Operation::NullOp);

        depgraph.register_node(op_id)
    }
}

