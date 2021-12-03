use log::warn;
use pyo3::prelude::*;

use painter_data::image::Image;
use painter_data::template::create_default_image;

use glam::Mat3;
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
                b: 1.0,
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
            self.insert_operation_onto = Some(new_op_id);
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
        let layer_existing_tips = self
            .image
            .depgraph
            .depends_on(&layer_blend_op_id)
            .expect("Unable to find blend op in depsgraph");
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

    /// Converts from screen coordinates (range -1, 1 on x/y) to the canvas coordinates
    /// as transformed by manipulate_canvas. This allows drawing on a rotated canvas.
    pub fn screen_coords_to_canvas_coords(&self, x: f32, y: f32) -> [f32; 2] {
        let invec = glam::Vec2::new(x, y);
        let transform = self.canvas_transform.to_mat().inverse();
        let vec = transform.transform_point2(invec);
        [vec.x, vec.y]
    }

    /// Sets the color that will be used by the various tools - brush, bucket fill etc.
    pub fn set_primary_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color.r = r;
        self.color.g = g;
        self.color.b = b;
        self.color.a = a;
    }

    /// Generates a string representation of the depgraph that can be visualized using
    /// dot https://en.wikipedia.org/wiki/DOT_(graph_description_language)
    /// This is useful for debugging
    pub fn generate_dotgraph(&self) -> String {
        use painter_data::id_map::IncrId;
        self.image.depgraph.generate_dotgraph(&|operation_id| {
            let operation = self.image.operations.get_unchecked(operation_id);
            match operation {
                Operation::Composite(_dat) => {
                    return format!("Operation {} {:?}", operation_id.val(), operation);
                }
                Operation::Tag(str) => {
                    return format!("Operation {} Tag({})", operation_id.val(), str);
                }
                Operation::Output(_id) => {
                    return format!("Operation {} {:?}", operation_id.val(), operation);
                }
                Operation::Stroke(_dat) => {
                    return format!("Operation {} Stroke", operation_id.val());
                }
            }
        })
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
            translation,
        }
    }
}

impl CanvasTransform {
    pub fn to_mat(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(
            [self.zoom, self.zoom].into(),
            self.angle,
            self.translation.into(),
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
