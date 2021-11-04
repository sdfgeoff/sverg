use log::warn;
use pyo3::prelude::*;

use painter_data::image::Image;
use painter_data::template::create_default_image;

use painter_data::color_primitives::Color;
use painter_data::id_map::{IdMapBase, LayerId, OperationId};
use painter_data::operation::Operation;

#[pyclass]
#[derive(Clone)]
pub struct EditContext {
    #[pyo3(get)]
    pub image: Image,
    pub insert_operation_onto: Option<OperationId>,

    #[pyo3(get, set)]
    pub color: Color,
}

impl Default for EditContext {
    fn default() -> Self {
        let image = create_default_image();

        EditContext {
            image,
            insert_operation_onto: None,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

#[pymethods]
impl EditContext {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self::default())
    }

    pub fn insert_operation(&mut self, operation: Operation) -> OperationId {
        let new_op_id = self.image.operations.insert(operation);
        if let Some(op_onto) = self.insert_operation_onto {
            self.image.depgraph.operate_on(new_op_id, op_onto);
            self.insert_operation_onto = Some(op_onto);
        } else {
            warn!("Created orphan operation - no known location to place in depgraph")
        }
        return new_op_id;
    }

    pub fn select_layer(&mut self, layer_id: LayerId) {
        let layer_blend_op_id = self
            .image
            .layers
            .get_unchecked(&layer_id)
            .blend_operation_id;
        let layer_existing_tips = self.image.depgraph.get_children_mut(layer_blend_op_id);
        if layer_existing_tips.len() != 2 {
            warn!("Malformed layer blend operation: incorrect number of dependencies");
            self.insert_operation_onto = None;
        } else {
            self.insert_operation_onto = Some(layer_existing_tips[0]);
        }
    }
}
