use glow::HasContext;
use libc::RTLD_NOW;
use pyo3::prelude::*;
use std::ffi;



use painter_data::image::Image;
use painter_data::layer::{Layer, LayerId};
use painter_data::operation::{Operation, OperationId};
use painter_data::color_primitives::{Color, BlendMode};
use painter_data::brush::{BrushId};
use painter_data::stroke::StrokeData;

#[pyclass]
struct PainterCore {
    gl: glow::Context,

    image: Image,

    operation_insert_point: Option<OperationId>,
}

#[pymethods]
impl PainterCore {
    #[new]
    pub fn new() -> PyResult<Self> {
        let gl = create_context();

        let image = Image::new();

        Ok(Self { gl, image, operation_insert_point: None})
    }

    pub fn new_file(&mut self) {
        self.image = Image::new();
    }

    pub fn add_layer(&mut self, name: String) -> LayerId {
        self.image.layers.insert(Layer {
            name,
            blend_operation: None
        })
    }

    // pub fn set_insert_point(&mut self, insert_point: OperationId) {

    // }

    pub fn create_stroke(&mut self, brush: BrushId, color: Color, blend_mode: BlendMode) -> OperationId {
        let operation = Operation::Stroke(StrokeData {
            color,
            brush,
            points: Vec::new(),
            blend_mode,
        });
        let operation_id = self.image.operations.insert(operation);
        // TODO: Insert into depgraph
        operation_id
    }

    // pub fn add_point_to_stroke(&mut self, stroke: OperationId, point: StrokePointData) {

    // }


    pub fn render(&mut self) -> PyResult<()> {
        println!("Rendering (rust)");
        unsafe {
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        Ok(())
    }

}


impl PainterCore {

}


fn create_context() -> glow::Context {
    println!("Attempting to grab openGL Context");

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
    println!("OpenGL Context Obtained!");
    gl
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn painter_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PainterCore>()?;
    Ok(())
}
