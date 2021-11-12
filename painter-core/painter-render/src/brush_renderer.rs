use super::quad;
use super::shader;
use glow::HasContext;
use painter_data::stroke::StrokeData;

use super::canvas::Canvas;

pub struct BrushRenderer {
    vertex_array_obj: glow::NativeVertexArray,
    brush_shader: shader::SimpleShader,
    vertex_position_buffer: glow::NativeBuffer,
    attrib_stroke_data: u32,
    stroke_data_buffer: glow::Buffer,
    uniform_aspect_ratio: glow::UniformLocation,
}

impl BrushRenderer {
    pub fn new(gl: &glow::Context) -> Self {
        let brush_shader = shader::SimpleShader::new(
            gl,
            include_str!("resources/brush.vert"),
            include_str!("resources/brush.frag"),
            "BrushRenderer",
        )
        .expect("Loading Brush Shader Failed");

        unsafe {
            let attrib_stroke_data = gl
                .get_attrib_location(brush_shader.program, "aStrokeData")
                .expect("Finding aStrokeData failed");

            let vertex_array_obj = gl
                .create_vertex_array()
                .expect("Failed creating vertex array");

            gl.bind_vertex_array(Some(vertex_array_obj));
            gl.object_label(
                glow::VERTEX_ARRAY,
                std::mem::transmute(vertex_array_obj),
                Some("BrushRendererVertexArray"),
            );

            let vertex_position_buffer = gl.create_buffer().expect("Failed creating vertex buffer");

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_position_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                quad::as_u8_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]),
                glow::STATIC_DRAW,
            );
            assert_eq!(gl.get_error(), glow::NO_ERROR);

            let stroke_data_buffer = gl
                .create_buffer()
                .expect("Failed to create BrushRenderer vertex buffer");

            
            let uniform_aspect_ratio =
            unsafe { gl.get_uniform_location(brush_shader.program, "aspectRatio") }
                .expect("Could not find uniform aspectRatio");


            gl.bind_vertex_array(None);
            Self {
                vertex_array_obj,
                uniform_aspect_ratio,
                vertex_position_buffer,
                brush_shader,
                attrib_stroke_data,
                stroke_data_buffer,
            }
        }
    }

    pub fn perform_stroke(&mut self, gl: &glow::Context, stroke: &StrokeData, canvas: &Canvas) {
        
        // We need all our point data layed out in a flat array
        let mut stroke_data_flat = Vec::with_capacity(stroke.points.len() * 4);
        for point in stroke.points.iter() {
            stroke_data_flat.push(point.position_x);
            stroke_data_flat.push(point.position_y);
            stroke_data_flat.push(point.pressure);
            stroke_data_flat.push(point.time);
        }
        
        unsafe {
            gl.push_debug_group(glow::DEBUG_SOURCE_APPLICATION, 0, "BrushRenderer");
            gl.bind_vertex_array(Some(self.vertex_array_obj));
        }

        canvas.make_active(gl);
        self.brush_shader.bind(gl);

        // Set up the mesh vertex position:
        unsafe {
            gl.enable_vertex_attrib_array(self.brush_shader.attrib_vertex_positions);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_position_buffer));

            gl.vertex_attrib_pointer_f32(
                self.brush_shader.attrib_vertex_positions, //index: u32,
                2,                       //size: i32,
                glow::FLOAT,                   //data_type: u32,
                false,                   //normalized: bool,
                0,                       //(core::mem::size_of::<f32>() * 2) as i32, //stride: i32,
                0,                       //offset: i32
            );
        }


        // Set up the brush stroke position data:
        unsafe {
            gl.enable_vertex_attrib_array(self.attrib_stroke_data);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.stroke_data_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                quad::as_u8_slice(&stroke_data_flat),
                glow::STATIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(
                self.attrib_stroke_data, //index: u32,
                4,                       //size: i32,
                glow::FLOAT,             //data_type: u32,
                false,                   //normalized: bool,
                0,                       //(core::mem::size_of::<f32>() * 2) as i32, //stride: i32,
                0,                       //offset: i32
            );
            gl.vertex_attrib_divisor(self.attrib_stroke_data, 1);
        }

        // Pass in contextual information
        unsafe {
            gl.uniform_1_f32(Some(&self.uniform_aspect_ratio), canvas.aspect_ratio())
        }

        unsafe {
            gl.draw_arrays_instanced(glow::TRIANGLE_STRIP, 0, 4, stroke.points.len() as i32);
        }

        unsafe {
            gl.pop_debug_group();
            gl.bind_vertex_array(None);
        }
    }
}
