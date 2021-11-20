use log::warn;
use pyo3::prelude::*;

use painter_data::color_primitives::BlendMode;
// use painter_data::color_primitives::Color;
use painter_data::brush::PressureSettings;
use painter_data::color_primitives::Color;
use painter_data::id_map::{BrushId, IdMapBase, OperationId};
use painter_data::operation::Operation;
use painter_data::stroke::StrokeData;

use super::context::EditContext;

#[pyclass]
#[derive(Clone)]
pub struct BrushTool {
    blend_mode: BlendMode,
    current_operation_id: Option<OperationId>,
    brush_id: Option<BrushId>,

    #[pyo3(get, set)]
    size: f32,
}

impl Default for BrushTool {
    fn default() -> Self {
        Self {
            blend_mode: BlendMode::Mix(1.0),
            current_operation_id: None,
            brush_id: None,
            size: 0.1,
        }
    }
}

#[pymethods]
impl BrushTool {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self::default())
    }

    pub fn set_brush_id(&mut self, brush_id: BrushId) {
        self.brush_id = Some(brush_id);
    }

    fn start_stroke(&mut self, context: &mut EditContext, x: f32, y: f32, pressure: f32) {
        if let Some(_op) = &self.current_operation_id {
            warn!(target: "brush_tool", "Starting stroke when one already exists");
        }

        match &self.brush_id {
            Some(brush_id) => {
                let operation = Operation::Stroke(StrokeData {
                    color: context.color.clone(),
                    glyph: brush_id.clone(),
                    position_array: Vec::new(),
                    angle_array: Vec::new(),
                    color_array: Vec::new(),
                    size: self.size,
                    size_array: Vec::new(),
                    blend_mode: self.blend_mode.clone(),
                });
                let operation_id = context.insert_operation(operation);
                self.current_operation_id = Some(operation_id);
                self.continue_stroke(context, x, y, pressure, 0.0);
            }
            None => {
                warn!(target: "brush_tool", "Brush tool does not have active brush")
            }
        }
    }

    fn continue_stroke(
        &mut self,
        context: &mut EditContext,
        x: f32,
        y: f32,
        pressure: f32,
        time_since_start: f32,
    ) {
        if let Some(operation_id) = &self.current_operation_id {
            let stroke = context.image.operations.get_mut_unchecked(operation_id);
            let brush = context
                .image
                .brushes
                .get_unchecked(self.brush_id.as_ref().expect("No Active Brush"));

            if let Operation::Stroke(stroke_data) = stroke {
                stroke_data.position_array.push([x, y]);
                stroke_data.angle_array.push(0.0);
                stroke_data.color_array.push(Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: evaluate_pressure_setting(&brush.flow, pressure),
                });
                stroke_data
                    .size_array
                    .push(evaluate_pressure_setting(&brush.size, pressure));
            } else {
                warn!(target: "brush_tool", "Current operation is not stroke");
            }
        } else {
            warn!(target: "brush_tool", "No stroke to draw into");
        }
    }

    fn end_stroke(&mut self) {
        self.current_operation_id = None;
    }
}

fn evaluate_pressure_setting(setting: &PressureSettings, pressure: f32) -> f32 {
    return setting.min_value + pressure * (setting.max_value - setting.min_value);
}
