use super::quad;
use super::shader;
use glow::HasContext;
use painter_data::stroke::StrokeData;

pub struct BrushRenderer {
    brush_shader: shader::SimpleShader,
    mesh: quad::Quad,
}

impl BrushRenderer {
    pub fn new(gl: &glow::Context) -> Self {
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

    pub fn perform_stroke(&mut self, gl: &glow::Context, stroke: &StrokeData) {
        println!("Drawing Stroke");

        self.brush_shader.bind(gl);
        self.mesh
            .bind(gl, self.brush_shader.attrib_vertex_positions);

        unsafe {
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }
}
