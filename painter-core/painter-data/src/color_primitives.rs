use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Color {
    #[pyo3(get)]
    pub r: f32,
    #[pyo3(get)]
    pub g: f32,
    #[pyo3(get)]
    pub b: f32,
    #[pyo3(get)]
    pub a: f32,
}

impl Color {
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

#[derive(FromPyObject, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum BlendMode {
    Mix(f32),
}
