use glow::HasContext;
use libc::RTLD_NOW;
use log::info;
use pyo3::prelude::*;
use std::ffi;

use log::warn;

use painter_data::id_map::IdMapBase;
use painter_tools::context::EditContext;

use painter_data::id_map::{OperationId, OperationIdMap};
use painter_data::operation::Operation;
use painter_depgraph::compute_execution;

mod brush_renderer;
mod canvas;
mod framebuffer_state;
mod gl_utils;
mod output_renderer;
mod quad;
mod shader;

use brush_renderer::BrushRenderer;
use output_renderer::OutputRenderer;

#[pyclass]
pub struct PainterRenderer {
    gl: glow::Context,
    brush_renderer: BrushRenderer,
    output_renderer: OutputRenderer,

    tmp_canvas: Option<canvas::Canvas>,
}

/// Returns the first zero-index output node in an OperationIdMap
fn get_output_node(operations: &OperationIdMap) -> Option<OperationId> {
    operations
        .iter()
        .filter_map(|(k, v)| {
            if *v == Operation::Output(0) {
                Some(k)
            } else {
                None
            }
        })
        .next()
        .cloned()
}

#[test]
fn test_get_output_node() {
    use painter_data::id_map::IdMapBase;
    use painter_data::id_map::OperationIdMap;
    use painter_data::operation::Operation;

    let mut map = OperationIdMap::default();
    let out_node_id = map.insert(Operation::Output(0));
    map.insert(Operation::Tag("test".to_string()));
    assert!(get_output_node(&map) == Some(out_node_id));

    let mut map = OperationIdMap::default();
    map.insert(Operation::Tag("test1".to_string()));
    map.insert(Operation::Tag("test2".to_string()));
    let out_node_id = map.insert(Operation::Output(0));
    map.insert(Operation::Tag("test3".to_string()));
    assert!(get_output_node(&map) == Some(out_node_id));
}

#[pymethods]
impl PainterRenderer {
    #[new]
    fn new() -> PyResult<Self> {
        let gl = create_gl_context();

        let brush_renderer = BrushRenderer::new(&gl);
        let output_renderer = OutputRenderer::new(&gl);

        Ok(Self {
            gl,
            brush_renderer,
            tmp_canvas: None,
            output_renderer,
        })
    }

    fn render(&mut self, context: &EditContext) {
        // let col = &context.image.metadata.canvas_background_color;
        let graph = &context.image.depgraph;

        let outp_framebuffer_state =
            framebuffer_state::FrameBufferState::from_current_gl_state(&self.gl);

        if let Some(canv) = self.tmp_canvas.as_mut() {
            canv.resize(&self.gl, context.image.metadata.preview_canvas_size);
            canv.make_active(&self.gl);
            unsafe {
                self.gl.clear_color(
                    context.image.metadata.canvas_background_color.r,
                    context.image.metadata.canvas_background_color.g,
                    context.image.metadata.canvas_background_color.b,
                    context.image.metadata.canvas_background_color.a,
                );
                self.gl.clear(glow::COLOR_BUFFER_BIT);
            }
        } else {
            self.tmp_canvas = Some(
                canvas::Canvas::new(
                    &self.gl,
                    context.image.metadata.preview_canvas_size,
                    "tmp_canvas",
                )
                .expect("Failed to create output canvas"),
            );
        }

        let output_node = get_output_node(&context.image.operations).expect("No Output Node");
        let order_of_operations = compute_execution(&context.image.depgraph, vec![output_node], 10).expect("Computing order of operation failed"); //graph.get_children_recursive_breadth_first(output_node);
        // From here we coud in theory remove any operations that haven't changed since last time and are in cache.
        // but for now that isn't implemented.
        for operation_stage in order_of_operations.iter() {
            match context.image.operations.get_unchecked(&operation_stage.operation.0.id) {
                Operation::Stroke(stroke_data) => {
                    if let Some(glyph) = context.image.glyphs.get(&stroke_data.glyph) {
                        self.brush_renderer.perform_stroke(
                            &self.gl,
                            stroke_data,
                            &glyph,
                            &self.tmp_canvas.as_ref().unwrap(),
                        );
                    } else {
                        warn!("Unable to find brush for stroke");
                    }
                }
                Operation::Output(_id) => {
                    let tmp_canvas = self.tmp_canvas.as_ref().unwrap();
                    self.output_renderer.render(
                        &self.gl,
                        context,
                        &tmp_canvas.texture,
                        &outp_framebuffer_state,
                    );
                }
                Operation::Tag(_name) => {}
                Operation::Composite(_name) => {}
            }
        }
    }
}

fn create_gl_context() -> glow::Context {
    info!("Attempting to grab openGL Context");

    let handle = unsafe {
        let h = libc::dlopen(
            ffi::CStr::from_bytes_with_nul_unchecked("libGL.so.1\0".as_bytes()).as_ptr(),
            RTLD_NOW,
        );
        if h.is_null() {
            eprintln!(
                "{}",
                ffi::CStr::from_ptr(libc::dlerror())
                    .to_string_lossy()
                    .as_ref()
            );
            std::process::exit(1);
        }
        h
    };

    let gl = unsafe {
        glow::Context::from_loader_function(|symbol| {
            let cst = ffi::CString::new(symbol).unwrap();
            libc::dlsym(handle, cst.as_ptr()) as *const _
        })
    };
    info!("OpenGL Context Obtained!");
    gl
}
