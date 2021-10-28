struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

enum ImageFormat {
    Png,
}


enum BlendMode {
    Mix(f32),
}

struct PressureSettings {
    min_value: f64,
    max_value: f64,
    random: f64,
}


struct Brush {
    bitmap: (ImageFormat, Vec<u8>),
    size: PressureSettings,
    flow: PressureSettings,
    scatter: PressureSettings,
    gap: PressureSettings,
}

struct Layer {
    name: String,
    tip_operation: OperationId,
}

// Making 1000 strokes per second we will run out of ID's
// in about 585 million years. I think that's enough
struct OperationId(u64),


struct StrokePoint {
    /// Position of this stroke on the horizontal axis
    position_x: f32,

    /// Position of this stroke on the vertical axis
    position_y: f32,

    /// How hard the user was pressing on the screen when this part of the stroke
    /// was drawn. Normalized between 0.0 and 1.0 with 1.0 being pushing really hard
    pressure: f32,
    
    /// Time since start of stroke in seconds
    time: f32,
}


struct StrokeData {
    color: Color,
    brush: BrushId,
    points: Vec<StrokePoint>
}

enum Operation {
    Stroke(StrokeData)
}

struct OperationNode {
    operation: Operation,
    blend_mode: BlendMode,
}


struct MetaData {
    preview_canvas_size: [i32, i32],
    canvas_background_color: Color,
}


struct File {
    file_format_version: (i32, i32, i32),
    brushes: Vec<Brush>
    depgraph: HashMap<OperationId, Vec<OperationId>>,
    layers: Vec<Layer>
    operations: HashMap<OperationId, OperationNode>,
    metadata: MetaData
}