use log::warn;
use pyo3::prelude::*;

use painter_data::image::Image;
use painter_data::template::create_default_image;

use painter_data::color_primitives::Color;
use painter_data::id_map::{IdMapBase, LayerId, OperationId};
use painter_data::operation::Operation;
use glam::Mat3;

#[pyclass]
#[derive(Clone)]
pub struct EditContext {
    #[pyo3(get)]
    pub image: Image,
    pub insert_operation_onto: Option<OperationId>,

    #[pyo3(get, set)]
    pub color: Color,

    #[pyo3(get, set)]
    pub canvas_transform: CanvasTransform,
}

impl Default for EditContext {
    fn default() -> Self {
        let image = create_default_image();
        Self::new_with_image(image)
    }
}

impl EditContext {
    pub fn new_with_image(image: Image) -> Self {
        EditContext {
            image,
            insert_operation_onto: None,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            canvas_transform: CanvasTransform::default(),
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

    pub fn manipulate_canvas(&mut self, zoom: f32, angle: f32, translation: [f32; 2]) {
        self.canvas_transform.zoom = zoom;
        self.canvas_transform.angle = angle;
        self.canvas_transform.translation = translation;

    }
}


#[pyclass]
#[derive(Clone)]
pub struct CanvasTransform {
    #[pyo3(get)]
    zoom: f32,
    #[pyo3(get)]
    angle: f32,
    #[pyo3(get)]
    translation: [f32; 2],
}

#[pymethods]
impl CanvasTransform {
    #[new]
    fn new(zoom: f32, angle: f32, translation: [f32; 2]) -> Self {
        Self {
            zoom,
            angle,
            translation
        }
    }
}

impl CanvasTransform {
    pub fn to_mat(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(
            [self.zoom, self.zoom].into(),
            self.angle,
            self.translation.into()
        )
    }
}

impl Default for CanvasTransform {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            angle: 0.0,
            translation: [0.0, 0.0],
        }
    }
}