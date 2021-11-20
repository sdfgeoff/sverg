use crate::color_primitives::{BlendMode, Color};
use crate::id_map::BrushId;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct StrokePoint {
    /// Position of this stroke on the horizontal axis
    #[pyo3(get, set)]
    pub position_x: f32,

    /// Position of this stroke on the vertical axis
    #[pyo3(get, set)]
    pub position_y: f32,

    /// How hard the user was pressing on the screen when this part of the stroke
    /// was drawn. Normalized between 0.0 and 1.0 with 1.0 being pushing really hard
    #[pyo3(get, set)]
    pub pressure: f32,

    /// The time at which this point was added to the stroke relative to the start of
    /// the stroke. (eg the first stroke will have time = 0)
    #[pyo3(get, set)]
    pub time: f32,
}

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct StrokeData {
    #[pyo3(get, set)]
    pub color: Color,


    #[pyo3(get, set)]
    pub brush: BrushId,
    #[pyo3(get, set)]
    pub points: Vec<StrokePoint>,

    pub blend_mode: BlendMode,
}
