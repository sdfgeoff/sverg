use super::framebuffer_state::FrameBufferState;
use super::gl_utils::texture_unit_id_to_gl;
use super::quad::Quad;
use super::shader::SimpleShader;
use glow::HasContext;
use painter_tools::context::EditContext;

/// Renders a texture output to the GTK/output framebuffer
pub struct OutputRenderer {
    output_shader: SimpleShader,
    mesh: Quad,
    uniform_screen_to_canvas: glow::UniformLocation,
    uniform_output_texture: glow::UniformLocation,
    uniform_screen_resolution: glow::UniformLocation,
    uniform_canvas_resolution: glow::UniformLocation,
}

impl OutputRenderer {
    pub fn new(gl: &glow::Context) -> Self {
        let output_shader = SimpleShader::new(
            gl,
            include_str!("resources/output.vert"),
            include_str!("resources/output.frag"),
        )
        .expect("Loading Output Shader Failed");

        let uniform_screen_to_canvas =
            unsafe { gl.get_uniform_location(output_shader.program, "screenToCanvas") }
                .expect("Could not find uniform screenToCanvas");
        let uniform_output_texture =
            unsafe { gl.get_uniform_location(output_shader.program, "outputTexture") }
                .expect("Could not find uniform outputTexture");

        let uniform_screen_resolution =
            unsafe { gl.get_uniform_location(output_shader.program, "screenResolution") }
                .expect("Could not find uniform screenResolution");
        let uniform_canvas_resolution =
            unsafe { gl.get_uniform_location(output_shader.program, "canvasResolution") }
                .expect("Could not find uniform canvasesolution");

        Self {
            mesh: Quad::new(gl).expect("Creating Output Mesh Failed"),
            output_shader,
            uniform_output_texture,
            uniform_screen_to_canvas,
            uniform_screen_resolution,
            uniform_canvas_resolution,
        }
    }
    pub fn render(
        &self,
        gl: &glow::Context,
        context: &EditContext,
        tex: &glow::Texture,
        output_state: &FrameBufferState,
    ) {
        output_state.apply(gl);

        self.output_shader.bind(gl);
        self.mesh
            .bind(gl, self.output_shader.attrib_vertex_positions);

        unsafe {
            // Bind our output texture
            let texture_unit_id = 0;
            gl.active_texture(texture_unit_id_to_gl(texture_unit_id));
            gl.bind_texture(glow::TEXTURE_2D, Some(*tex));
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.uniform_1_i32(Some(&self.uniform_output_texture), texture_unit_id as i32);

            // Transfer the canvas
            gl.uniform_matrix_3_f32_slice(
                Some(&self.uniform_screen_to_canvas),
                false,
                &context.canvas_transform.to_mat().to_cols_array(),
            );

            // Pass in the screen resolution and the canvas resolution
            gl.uniform_2_f32(
                Some(&self.uniform_canvas_resolution),
                context.image.metadata.preview_canvas_size[0] as f32,
                context.image.metadata.preview_canvas_size[1] as f32,
            );
            gl.uniform_2_f32(
                Some(&self.uniform_screen_resolution),
                output_state.resolution[2] as f32,
                output_state.resolution[3] as f32,
            );

            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }
}
