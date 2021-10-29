
use serde::{Serialize, Deserialize};
use pyo3::prelude::*;


#[pyclass]
#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}


#[derive(FromPyObject)]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum BlendMode {
    Mix(f32),
}