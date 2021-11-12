/// LOOK AT:
/// https://rust-tutorials.github.io/learn-opengl/basics/001-drawing-a-triangle.html
use glow::{Buffer, Context, HasContext, ARRAY_BUFFER, FLOAT, STATIC_DRAW};

/// An error with this whole object.
#[derive(Debug)]
pub enum QuadError {
    /// Failed to upload buffer data to the GPU
    BufferCreationFailed(String),
}

/// A four-vertex mesh reaching from 0.0 to 1.0 on each axis
pub struct Quad {
    vertex_array_obj: glow::NativeVertexArray,
    position_buffer: Buffer,
}

impl Quad {
    pub fn new(gl: &Context) -> Result<Self, QuadError> {
        
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
            as_u8_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]),
            glow::STATIC_DRAW,
        );
        assert_eq!(gl.get_error(), glow::NO_ERROR);
    }

    unsafe {
        gl.bind_vertex_array(None);
    }

        Ok(Self { position_buffer, vertex_array_obj })
    }

    pub fn bind(&self, gl: &Context, attrib_vertex_positions: u32) {
        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array_obj));

            gl.enable_vertex_attrib_array(attrib_vertex_positions);
            gl.bind_buffer(ARRAY_BUFFER, Some(self.position_buffer));

            gl.vertex_attrib_pointer_f32(
                attrib_vertex_positions, //index: u32,
                2,                       //size: i32,
                FLOAT,                   //data_type: u32,
                false,                   //normalized: bool,
                0,                       //(core::mem::size_of::<f32>() * 2) as i32, //stride: i32,
                0,                       //offset: i32
            );
        }
    }
}


pub fn as_u8_slice(v: &[f32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}
