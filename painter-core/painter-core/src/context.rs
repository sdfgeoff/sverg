use pyo3::prelude::*;

use painter_data::image::Image;
use painter_data::template::create_default_image;

use painter_data::color_primitives::Color;
use painter_data::id_map::{LayerId, OperationId, IdMapBase};

use painter_data::layer::Layer;

#[pyclass]
#[derive(Clone)]
pub struct EditContext {
    #[pyo3(get)]
    pub image: Image,
    pub operation_insert_point: Option<OperationId>,

    #[pyo3(get,set)]
    pub color: Color,
}


impl Default for EditContext {
    fn default() -> Self {
        let image = create_default_image();

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
            blend_operation_id: None,
        })
        // TODO Insert into depsgraph
    }
}