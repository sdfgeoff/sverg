use pyo3::prelude::*;

use painter_data::image::Image;
use painter_data::operation::OperationId;

use painter_data::color_primitives::Color;


use painter_data::layer::{Layer, LayerId};

#[pyclass]
#[derive(Clone)]
pub struct EditContext {
    pub image: Image,
    pub operation_insert_point: Option<OperationId>,
    pub color: Color,
}


impl Default for EditContext {
    fn default() -> Self {
        let image = Image::default();

        EditContext {
            image,
            operation_insert_point: None,
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

    pub fn add_layer(&mut self, name: String) -> LayerId {
        self.image.layers.insert(Layer {
            name,
            blend_operation: None,
        })
        // TODO Insert into depsgraph
    }
}