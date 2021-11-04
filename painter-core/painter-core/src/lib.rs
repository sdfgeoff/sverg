use pyo3::prelude::*;

use painter_tools::brush_tool::BrushTool;
use painter_tools::context::EditContext;

use simple_logger::SimpleLogger;

use painter_render::PainterRenderer;

#[pyclass]
struct PainterCore {}

#[pymethods]
impl PainterCore {
    #[new]
    pub fn new() -> PyResult<Self> {
        SimpleLogger::new().init().unwrap();

        Ok(Self {})
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
