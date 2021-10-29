use crate::color_primitives::Color;
use crate::layer::Layer;
use crate::brush::{Brush, PressureSettings, BrushGlyph};
use crate::image::{Image, MetaData};
use crate::id_map::{OperationIdMap, LayerIdMap, BrushIdMap, IdMapBase};

use std::collections::HashMap;


pub fn create_default_image() -> Image {
    let mut image = Image {
        file_format_version: (0, 0, 1),
        brushes: BrushIdMap::default(),
        operations: OperationIdMap::default(),
        depgraph: HashMap::new(),
        layers: LayerIdMap::default(),
        metadata: MetaData {
            preview_canvas_size: [1920, 1080],
            canvas_background_color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        },
    };

    image.brushes.insert(Brush {
        bitmap: BrushGlyph::Png(include_bytes!("resources/spiral.png").to_vec()),
        size: PressureSettings {
            min_value: 0.0,
            max_value: 0.1,
            random: 0.0,
        },
        flow: PressureSettings {
            min_value: 0.5,
            max_value: 1.0,
            random: 0.0,
        },
        scatter: PressureSettings {
            min_value: 0.0,
            max_value: 0.0,
            random: 0.0,
        },
        gap: PressureSettings {
            min_value: 0.0,
            max_value: 0.0,
            random: 0.0,
        },
    });

    image.layers.insert(Layer {
        name: "Background".to_string(),
        blend_operation_id: None,
    });

    image
}