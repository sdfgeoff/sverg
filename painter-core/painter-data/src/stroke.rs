use serde::{Serialize, Deserialize};
use crate::color_primitives::{BlendMode, Color};
use crate::brush::BrushId;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct StrokePoint {
    /// Position of this stroke on the horizontal axis
    position_x: f32,

    /// Position of this stroke on the vertical axis
    position_y: f32,

    /// How hard the user was pressing on the screen when this part of the stroke
    /// was drawn. Normalized between 0.0 and 1.0 with 1.0 being pushing really hard
    pressure: f32,

    /// Time since start of stroke in seconds
    time: f32,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct StrokeData {
    pub color: Color,
    pub brush: BrushId,
    pub points: Vec<StrokePoint>,
    pub blend_mode: BlendMode,
}