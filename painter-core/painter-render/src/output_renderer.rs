use super::framebuffer_state::FrameBufferState;
use super::gl_utils::texture_unit_id_to_gl;
use super::quad;
use super::shader::SimpleShader;
use glow::HasContext;
use painter_tools::context::EditContext;

/// Renders a texture output to the GTK/output framebuffer
pub struct OutputRenderer {
    output_shader: SimpleShader,
    uniform_screen_to_canvas: glow::UniformLocation,
    uniform_output_texture: glow::UniformLocation,
    uniform_screen_resolution: glow::UniformLocation,
    uniform_canvas_resolution: glow::UniformLocation,
    position_buffer: glow::NativeBuffer,
    vertex_array_obj: glow::NativeVertexArray,
}

impl OutputRenderer {
    pub fn new(gl: &glow::Context) -> Self {
        let output_shader = SimpleShader::new(
            gl,
            include_str!("resources/output.vert"),
            include_str!("resources/output.frag"),
            "OutputRenderer"
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

        let vertex_array_obj =
            unsafe { gl.create_vertex_array() }.expect("Failed creating vertex array");
        unsafe {
            gl.bind_vertex_array(Some(vertex_array_obj));
            gl.object_label(glow::VERTEX_ARRAY, std::mem::transmute(vertex_array_obj), Some("OutputRenderVertexArray"));
        }

        let position_buffer = unsafe { gl.create_buffer() }.expect("Failed creating vertex buffer");
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(position_buffer));
            // gl.object_label(glow::ARRAY_BUFFER, std::mem::transmute(position_buffer), Some("OutputRenderVertexPositionBuffer"));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                quad::as_u8_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]),
                glow::STATIC_DRAW,
            );
            assert_eq!(gl.get_error(), glow::NO_ERROR);
        }

        unsafe {
            gl.bind_vertex_array(None);
        }

        Self {
            output_shader,
            uniform_output_texture,
            uniform_screen_to_canvas,
            uniform_screen_resolution,
            uniform_canvas_resolution,
            position_buffer,
            vertex_array_obj,
        }
    }
    pub fn render(
        &self,
        gl: &glow::Context,
        context: &EditContext,
        tex: &glow::Texture,
        output_state: &FrameBufferState,
    ) {
        unsafe {
            gl.push_debug_group(glow::DEBUG_SOURCE_APPLICATION, 0, "OutputRenderer");
        }

        // Set up our output
        output_state.apply(gl);
        unsafe {
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        // Prep for rendering
        self.output_shader.bind(gl);

        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array_obj));
                 
            gl.enable_vertex_attrib_array(self.output_shader.attrib_vertex_positions);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.position_buffer));
            gl.vertex_attrib_pointer_f32(
                self.output_shader.attrib_vertex_positions, //index: u32,
                2,                                     //size: i32,
                glow::FLOAT,                           //data_type: u32,
                false,                                 //normalized: bool,
                0, //(core::mem::size_of::<f32>() * 2) as i32, //stride: i32,
                0, //offset: i32
            );
        }

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

            // Do the rendering
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }

        unsafe {
            gl.bind_vertex_array(None);
        }


        unsafe {
            gl.pop_debug_group();
        }
    }
}
