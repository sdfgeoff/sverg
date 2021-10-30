use glow::HasContext;
use libc::RTLD_NOW;
use pyo3::prelude::*;
use std::ffi;

use painter_tools::context::EditContext;

#[pyclass]
pub struct PainterRenderer {
    gl: glow::Context,
}


#[pymethods]
impl PainterRenderer {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            gl: create_gl_context()
        })
    }

    fn render(&mut self, context: EditContext) {
        println!("Rendering (rust)");
        let col = context.image.metadata.canvas_background_color;
        unsafe {
            self.gl.clear_color(col.r, col.g, col.b, col.a);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }
}



fn create_gl_context() -> glow::Context {
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