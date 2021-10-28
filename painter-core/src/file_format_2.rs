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
    operations: Vec<Operation>,
    name: String,
}

struct Composite {
    layer1: Layer,
    layer2: Layer,
    blend_mode: BlendMode,
}

struct MetaData {
    preview_canvas_size: [i32, i32],
}

struct Image {
    file_format_version: (u32, u32, u32)
    metadata: MetaData,

}