use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::id_map::AddIncr;

#[pyclass]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BrushId(u64);

impl AddIncr for BrushId {
    fn increment(&mut self) -> Self {
        let out = Self(self.0);
        self.0 += 1;
        out
    }
}

impl BrushId {
    pub fn new() -> Self {
        BrushId(0)
    }
}

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
