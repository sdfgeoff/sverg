use super::quad;
use super::shader;
use glow::HasContext;
use painter_data::stroke::StrokeData;

use super::canvas::Canvas;

pub struct BrushRenderer {
    brush_shader: shader::SimpleShader,
    mesh: quad::Quad,
    attrib_stroke_data: u32,
    stroke_data_buffer: glow::Buffer,
}

impl BrushRenderer {
    pub fn new(gl: &glow::Context) -> Self {
        let brush_shader = shader::SimpleShader::new(
            gl,
            include_str!("resources/brush.vert"),
            include_str!("resources/brush.frag"),
        )
        .expect("Loading Brush Shader Failed");
        let attrib_stroke_data =
            unsafe { gl.get_attrib_location(brush_shader.program, "aStrokeData") }
                .expect("Finding aStrokeData failed");

        let stroke_data_buffer = unsafe {
            // let vao = gl
            //     .create_vertex_array()
            //     .expect("Failed to create BrushRenderer vertex array");
            // gl.bind_vertex_array(Some(vao));
            let vbo = gl
                .create_buffer()
                .expect("Failed to create BrushRenderer vertex buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            vbo
        };

        Self {
            mesh: quad::Quad::new(gl).expect("Creating Brush Mesh Failed"),
            brush_shader,
            attrib_stroke_data,
            stroke_data_buffer,
        }
    }

    pub fn perform_stroke(&mut self, gl: &glow::Context, stroke: &StrokeData, canvas: &Canvas) {
        canvas.make_active(gl);

        self.brush_shader.bind(gl);
        self.mesh
            .bind(gl, self.brush_shader.attrib_vertex_positions);


        let mut stroke_data_flat = Vec::with_capacity(stroke.points.len() * 4);
        for point in stroke.points.iter() {
            stroke_data_flat.push(point.position_x);
            stroke_data_flat.push(point.position_y);
            stroke_data_flat.push(point.pressure);
            stroke_data_flat.push(0.0);
        }
        

        unsafe {
            // TODO: I have no idea what I am doing here. I suspect I'm minging my VAO's, but I'm really not sure

            gl.enable_vertex_attrib_array(self.attrib_stroke_data);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.stroke_data_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, quad::as_u8_slice(&stroke_data_flat), glow::STATIC_DRAW);

            gl.vertex_attrib_divisor(self.attrib_stroke_data, 4);
            gl.vertex_attrib_pointer_f32(
                self.attrib_stroke_data, //index: u32,
                4,                       //size: i32,
                glow::FLOAT,                   //data_type: u32,
                false,                   //normalized: bool,
                0,                       //(core::mem::size_of::<f32>() * 2) as i32, //stride: i32,
                0,                       //offset: i32
            );
            
        }

        unsafe {
            gl.draw_arrays_instanced(glow::TRIANGLE_STRIP, 0, 4, stroke.points.len() as i32);
        }
    }
}

