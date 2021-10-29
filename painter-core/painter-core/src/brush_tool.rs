use pyo3::prelude::*;
use log::warn;

use painter_data::color_primitives::BlendMode;
// use painter_data::color_primitives::Color;
use painter_data::stroke::{StrokeData, StrokePoint};
use painter_data::brush::{BrushId};
use painter_data::operation::{OperationId, Operation};

use super::EditContext;


#[pyclass]
#[derive(Clone)]
pub struct BrushTool {
    blend_mode: BlendMode,
    current_operation_id: Option<OperationId>,
    brush_id: Option<BrushId>,
}


impl Default for BrushTool {
    fn default() -> Self {
        Self {
            blend_mode: BlendMode::Mix(1.0),
            current_operation_id: None,
            brush_id: None,
        }
    }
}

#[pymethods]
impl BrushTool {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self::default())
    }

    fn start_stroke(&mut self, context: &mut EditContext, x: f32, y: f32, pressure: f32) {
        if let Some(_op) = &self.current_operation_id {
            warn!(target: "brush_tool", "Starting stroke when one already exists");
        }

        match &self.brush_id {
            Some(brush_id) => {
                let operation = Operation::Stroke(StrokeData {
                    color: context.color.clone(),
                    brush: brush_id.clone(),
                    points: Vec::new(),
                    blend_mode: self.blend_mode.clone(),
        
                });
                let operation_id = context.image.operations.insert(operation);
        
                self.current_operation_id = Some(operation_id);

                self.continue_stroke(context, x, y, pressure);
            }
            None => {
                warn!(target: "brush_tool", "Brush tool does not have active brush")
            }
        }
    }

    fn continue_stroke(&mut self, context: &mut EditContext, x: f32, y: f32, pressure: f32) {
        if let Some(operation_id) = &self.current_operation_id {
            let stroke = context.image.operations.get_mut_unchecked(operation_id);
            if let Operation::Stroke(stroke_data) = stroke {
                stroke_data.points.push(StrokePoint {
                    position_x: x,
                    position_y: y,
                    pressure,
                })
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