use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::color_primitives::BlendMode;
use crate::stroke::StrokeData;

#[derive(FromPyObject, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Stroke(StrokeData),
    Composite(BlendMode),
}
