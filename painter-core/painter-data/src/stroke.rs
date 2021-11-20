use crate::color_primitives::{BlendMode, Color};
use crate::id_map::BrushId;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct StrokeData {
    pub position_array: Vec<[f32; 2]>,
    pub angle_array: Vec<f32>,

    pub size: f32,
    pub size_array: Vec<f32>,

    pub color: Color,
    pub color_array: Vec<Color>,

    pub glyph: BrushId,
    pub blend_mode: BlendMode,
}
