use pyo3::prelude::*;
use serde::{Deserialize, Serialize};


#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PressureSettings {
    pub min_value: f64,
    pub max_value: f64,
    pub random: f64,
}

#[pyclass]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Brush {
    pub bitmap: BrushGlyph,
    pub size: PressureSettings,
    pub flow: PressureSettings,
    pub scatter: PressureSettings,
    pub gap: PressureSettings,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum BrushGlyph {
    Png(Vec<u8>),
}
