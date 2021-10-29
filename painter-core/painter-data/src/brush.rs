use serde::{Serialize, Deserialize};
use pyo3::prelude::*;

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
    pub fn new() -> Self{
        BrushId(0)
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PressureSettings {
    min_value: f64,
    max_value: f64,
    random: f64,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Brush {
    bitmap: BrushGlyph,
    size: PressureSettings,
    flow: PressureSettings,
    scatter: PressureSettings,
    gap: PressureSettings,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum BrushGlyph {
    Png(Vec<u8>)
}