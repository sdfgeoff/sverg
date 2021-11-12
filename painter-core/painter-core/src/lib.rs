use pyo3::prelude::*;

use painter_tools::brush_tool::BrushTool;
use painter_tools::context::EditContext;

use simple_logger::SimpleLogger;

use painter_render::PainterRenderer;
use painter_data::{write_into, load_from_reader};

#[pyclass]
struct PainterCore {
}

#[pymethods]
impl PainterCore {
    #[new]
    pub fn new(py: Python) -> PyResult<Self> {
        SimpleLogger::new().init().unwrap();

        Ok(Self {
        })
    }

    pub fn save(&self, context: EditContext, filename: String) {
        let buffer = std::fs::File::create(filename).expect("Failed to create file");
        write_into(&context.image, buffer).expect("Failed to write");
    }

    pub fn load(&self, filename: String) -> EditContext {
        let buffer = std::fs::File::open(filename).expect("Failed to open file");
        let image = load_from_reader(buffer).expect("Failed to read");
        EditContext::new_with_image(image)
    }

    #[staticmethod]
    pub fn new_image() -> EditContext {
        EditContext::default()
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn painter_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PainterCore>()?;
    m.add_class::<BrushTool>()?;
    m.add_class::<EditContext>()?;
    m.add_class::<PainterRenderer>()?;
    Ok(())
}
