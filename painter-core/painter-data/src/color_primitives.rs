use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Color {
    #[pyo3(get, set)]
    pub r: f32,
    #[pyo3(get, set)]
    pub g: f32,
    #[pyo3(get, set)]
    pub b: f32,
    #[pyo3(get, set)]
    pub a: f32,
}

#[derive(FromPyObject, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum BlendMode {
    Mix(f32),
}
