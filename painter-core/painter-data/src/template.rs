use crate::brush::{Brush, Glyph, PressureSettings};
use crate::color_primitives::{BlendMode, Color};
// use crate::depgraph::DepGraph;
use crate::id_map::{BrushIdMap, GlyphIdMap, IdMapBase, LayerIdMap, OperationIdMap};
use crate::image::{Image, MetaData};
use crate::layer::Layer;
use crate::operation::Operation;
use painter_depgraph::DepGraph;

pub fn create_default_image() -> Image {
    let mut image = Image {
        brushes: BrushIdMap::default(),
        operations: OperationIdMap::default(),
        glyphs: GlyphIdMap::default(),
        layers: LayerIdMap::default(),

        depgraph: DepGraph::default(),
        metadata: MetaData {
            preview_canvas_size: [1920 * 2, 1080 * 2],
            canvas_background_color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        },
    };

    let output_op_id = image.operations.insert(Operation::Output(0));
    let background_blend_op_id = image
        .operations
        .insert(Operation::Composite(BlendMode::Mix(1.0)));

    image.layers.insert(Layer {
        name: "Background".to_string(),
        blend_operation_id: background_blend_op_id.clone(),
    });
    let canvas_base = image
        .operations
        .insert(Operation::Tag("CanvasBase".to_string()));
    let background_layer_start = image
        .operations
        .insert(Operation::Tag("BackgroundLayerStart".to_string()));

    image.depgraph.insert(background_layer_start, vec![]);
    image.depgraph.insert(canvas_base, vec![]);
    image
        .depgraph
        .insert(output_op_id, vec![background_blend_op_id]);
    image.depgraph.insert(
        background_blend_op_id,
        vec![background_layer_start, canvas_base],
    );

    image.brushes.insert(Brush {
        name: "Spiral".to_string(),
        glyph: Glyph::Png(include_bytes!("resources/spiral.png").to_vec()),
        size: PressureSettings {
            min_value: 0.0,
            max_value: 1.0,
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

    image
}
