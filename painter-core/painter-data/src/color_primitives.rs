use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(FromPyObject, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum BlendMode {
    Mix(f32),
}
