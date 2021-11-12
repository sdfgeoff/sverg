use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::color_primitives::BlendMode;
use crate::stroke::StrokeData;

#[derive(FromPyObject, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Stroke(StrokeData),
    Composite(BlendMode),

    // Number is just so it can be communicated to/from python, it's value is not used
    Output(u32),

    /// Operation does nothing to the canvas, but is useful to reference a location or to
    /// ensure an operation is present.
    Tag(String),
}
