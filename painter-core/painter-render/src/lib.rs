use glow::HasContext;
use libc::RTLD_NOW;
use log::info;
use pyo3::prelude::*;
use std::ffi;

use painter_data::id_map::IdMapBase;
use painter_tools::context::EditContext;

use std::collections::HashMap;

use painter_data::depgraph::DepGraph;
use painter_data::id_map::{OperationId, OperationIdMap};
use painter_data::operation::Operation;
use painter_data::stroke::StrokeData;

mod canvas;
mod quad;
mod shader;

#[pyclass]
pub struct PainterRenderer {
    gl: glow::Context,

    brush_operator: BrushOperator,
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
    use painter_data::id_map::{OperationId, OperationIdMap};
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

        let brush_operator = BrushOperator::new(&gl);
        Ok(Self { gl, brush_operator })
    }

    fn render(&mut self, context: &EditContext) {
        println!("Rendering (rust)");
        let col = &context.image.metadata.canvas_background_color;
        let graph = &context.image.depgraph;

        let output_node = get_output_node(&context.image.operations).expect("No Output Node");
        let mut order_of_operations = graph.get_children_recursive_breadth_first(output_node);
        order_of_operations.reverse();

        // From here we coud in theory remove any operations that haven't changed since last time and are in cache.
        // but for now that isn't implemented.

        unsafe {
            self.gl.clear_color(col.r, col.g, col.b, col.a);
            self.gl.disable(glow::DEPTH);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        for operation_id in order_of_operations.iter() {
            match context.image.operations.get_unchecked(operation_id) {
                Operation::Stroke(stroke_data) => {
                    self.brush_operator.perform_stroke(&self.gl, stroke_data);
                }
                Operation::Output(_id) => {}
                Operation::Tag(_name) => {}
                Operation::Composite(_name) => {}
            }
        }
    }
}

use painter_data::id_map::BrushId;

struct BrushOperator {
    brush_shader: shader::SimpleShader,
    mesh: quad::Quad,
}

impl BrushOperator {
    fn new(gl: &glow::Context) -> Self {
        Self {
            mesh: quad::Quad::new(gl).expect("Creating Brush Mesh Failed"),
            brush_shader: shader::SimpleShader::new(
                gl,
                include_str!("resources/brush.vert"),
                include_str!("resources/brush.frag"),
            )
            .expect("Loading Brush Shader Failed"),
        }
    }

    fn perform_stroke(&mut self, gl: &glow::Context, stroke: &StrokeData) {
        println!("Drawing Stroke");

        self.brush_shader.bind(gl);
        self.mesh
            .bind(gl, self.brush_shader.attrib_vertex_positions);

        unsafe {
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
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
