use glow::HasContext;
use super::framebuffer_state::FrameBufferState;
use super::shader::SimpleShader;
use super::quad::Quad;
use painter_tools::context::EditContext;

/// Renders a texture output to the GTK/output framebuffer
pub struct OutputRenderer {
    output_shader: SimpleShader,
    mesh: Quad,
}

impl OutputRenderer {
    pub fn new(gl: &glow::Context) -> Self {
        Self {
            mesh: Quad::new(gl).expect("Creating Output Mesh Failed"),
            output_shader: SimpleShader::new(
                gl,
                include_str!("resources/brush.vert"),
                include_str!("resources/brush.frag"),
            )
            .expect("Loading Output Shader Failed"),
        }

    }
    pub fn render(&self, gl: &glow::Context, context: &EditContext, tex: &glow::Texture, output_state: &FrameBufferState) {
        output_state.apply(gl);

        self.output_shader.bind(gl);
        self.mesh
            .bind(gl, self.output_shader.attrib_vertex_positions);

        
        unsafe {    
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }
}

