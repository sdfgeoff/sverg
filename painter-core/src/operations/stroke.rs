use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct PressureSetting {
    min: f32,
    max: f32,
    random: f32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SpacingSettings {
    gap: f32,
    gap_variance: f32,
    scatter: f32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Brush {
    size: PressureSetting,
    opacity: PressureSetting,
    scatter: PressureSetting,
    gap: PressureSetting,
}

#[derive(PartialEq, Debug, Clone)]
pub struct StrokePoint {
    position: [f32; 2],
    pressure: f32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Stroke {
    brush: Rc<Brush>,
    points: Vec<StrokePoint>,
    color: [f32; 4]
}