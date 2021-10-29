use crate::id_map::BrushId;
use crate::color_primitives::{BlendMode, Color};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct StrokePoint {
    /// Position of this stroke on the horizontal axis
    pub position_x: f32,

    /// Position of this stroke on the vertical axis
    pub position_y: f32,

    /// How hard the user was pressing on the screen when this part of the stroke
    /// was drawn. Normalized between 0.0 and 1.0 with 1.0 being pushing really hard
    pub pressure: f32,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct StrokeData {
    pub color: Color,
    pub brush: BrushId,
    pub points: Vec<StrokePoint>,
    pub blend_mode: BlendMode,
}
