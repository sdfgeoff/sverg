use pyo3::prelude::*;
use glow::HasContext;
use libc::RTLD_NOW;
use std::ffi;
use std::rc::Rc;

mod operations;
mod image;

#[pyclass]
struct PainterCore {
    gl: glow::Context,

    #[pyo3(get)]
    image: image::Image,
}

#[pymethods]
impl PainterCore {
    #[new]
    pub fn new() -> PyResult<Self> {
        let gl = create_context();

        let image = image::Image::new();

        Ok(Self { gl, image })
    }

    pub fn render(&mut self) -> PyResult<()> {
        println!("Rendering (rust)");
        unsafe {
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        Ok(())
    }
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
