use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PressureSettings {
    pub min_value: f32,
    pub max_value: f32,
    pub random: f32,
}

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Brush {
    pub name: String,
    pub glyph: Glyph,
    pub size: PressureSettings,
    pub flow: PressureSettings,
    pub scatter: PressureSettings,
    pub gap: PressureSettings,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Hash, Eq)]
pub enum Glyph {
    Png(Vec<u8>),
}
