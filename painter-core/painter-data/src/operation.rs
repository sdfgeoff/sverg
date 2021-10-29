use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::stroke::StrokeData;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Stroke(StrokeData),
}
