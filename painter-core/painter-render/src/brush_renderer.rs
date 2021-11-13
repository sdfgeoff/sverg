use super::gl_utils;
use super::quad;
use super::shader;
use glow::HasContext;
use painter_data::brush::{Brush, BrushGlyph};
use painter_data::stroke::StrokeData;

use log::info;

use std::collections::HashMap;

use png::{BitDepth, ColorType};
use png;

use super::canvas::Canvas;

pub struct BrushRenderer {
    brush_texture_store: HashMap<BrushGlyph, glow::Texture>,

    vertex_array_obj: glow::NativeVertexArray,
    brush_shader: shader::SimpleShader,
    vertex_position_buffer: glow::NativeBuffer,
    attrib_stroke_data: u32,
    stroke_data_buffer: glow::Buffer,

    uniform_aspect_ratio: glow::UniformLocation,
    uniform_brush_size: glow::UniformLocation,
    uniform_brush_flow: glow::UniformLocation,
    uniform_brush_color: glow::UniformLocation,
    uniform_brush_texture: glow::UniformLocation,
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

            let uniform_aspect_ratio = gl
                .get_uniform_location(brush_shader.program, "aspectRatio")
                .expect("Could not find uniform aspectRatio");

            let uniform_brush_size = gl
                .get_uniform_location(brush_shader.program, "brushSize")
                .expect("Could not find uniform brushSize");
            let uniform_brush_flow = gl
                .get_uniform_location(brush_shader.program, "brushFlow")
                .expect("Could not find uniform brushFlow");
            let uniform_brush_color = gl
                .get_uniform_location(brush_shader.program, "brushColor")
                .expect("Could not find uniform brushColor");
            let uniform_brush_texture = gl
                .get_uniform_location(brush_shader.program, "brushTexture")
                .expect("Could not find uniform brushTexture");

            gl.bind_vertex_array(None);
            Self {
                brush_texture_store: HashMap::new(),

                vertex_array_obj,
                vertex_position_buffer,
                brush_shader,
                attrib_stroke_data,
                stroke_data_buffer,

                uniform_aspect_ratio,
                uniform_brush_size,
                uniform_brush_flow,
                uniform_brush_color,
                uniform_brush_texture,
            }
        }
    }

    /// Ensures a brushes texture is loaded onto the GPU and returns a reference to it
    pub fn load_brush_texture(&mut self, gl: &glow::Context, brush: &Brush) -> glow::Texture {
        if let Some(tex) = self.brush_texture_store.get(&brush.bitmap) {
            tex.clone()
        } else {
            info!("loading_brush_texture_to_gpu");
            unsafe {
                gl.push_debug_group(
                    glow::DEBUG_SOURCE_APPLICATION,
                    0,
                    &format!("LoadBrushTexture{}", brush.name),
                );
            }
            let new_tex = unsafe {
                gl.create_texture()
                    .expect("Failed to create texture for brush")
            };

            load_brush_into_texture(gl, brush, &new_tex);

            unsafe {
                gl.pop_debug_group();
            }

            self.brush_texture_store
                .insert(brush.bitmap.clone(), new_tex);
            self.brush_texture_store.get(&brush.bitmap).unwrap().clone()
        }
    }

    pub fn perform_stroke(
        &mut self,
        gl: &glow::Context,
        stroke: &StrokeData,
        brush: &Brush,
        canvas: &Canvas,
    ) {
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
            gl.enable(glow::BLEND);
            gl.blend_equation(glow::FUNC_ADD);
            gl.blend_func_separate(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA, glow::SRC1_ALPHA, glow::ONE);
        }


        let brush_texture = self.load_brush_texture(gl, brush);

        canvas.make_active(gl);
        self.brush_shader.bind(gl);


        // Set up the mesh vertex position:
        unsafe {
            gl.enable_vertex_attrib_array(self.brush_shader.attrib_vertex_positions);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_position_buffer));

            gl.vertex_attrib_pointer_f32(
                self.brush_shader.attrib_vertex_positions, //index: u32,
                2,                                         //size: i32,
                glow::FLOAT,                               //data_type: u32,
                false,                                     //normalized: bool,
                0, //(core::mem::size_of::<f32>() * 2) as i32, //stride: i32,
                0, //offset: i32
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
        unsafe { gl.uniform_1_f32(Some(&self.uniform_aspect_ratio), canvas.aspect_ratio()) }

        // Pass in brush parameters
        unsafe {
            gl.uniform_3_f32(
                Some(&self.uniform_brush_size),
                brush.size.min_value as f32,
                brush.size.max_value as f32,
                brush.size.random as f32,
            );
            gl.uniform_3_f32(
                Some(&self.uniform_brush_flow),
                brush.flow.min_value as f32,
                brush.flow.max_value as f32,
                brush.flow.random as f32,
            );
            gl.uniform_4_f32(
                Some(&self.uniform_brush_color),
                stroke.color.r,
                stroke.color.g,
                stroke.color.b,
                stroke.color.a,
            );

            let brush_texture_unit_id = 0;
            gl.active_texture(gl_utils::texture_unit_id_to_gl(brush_texture_unit_id));
            gl.bind_texture(glow::TEXTURE_2D, Some(brush_texture));
            gl.uniform_1_i32(Some(&self.uniform_brush_texture), brush_texture_unit_id as i32);
        }

        unsafe {
            gl.draw_arrays_instanced(glow::TRIANGLE_STRIP, 0, 4, stroke.points.len() as i32);
        }

        unsafe {
            gl.bind_vertex_array(None);
            gl.disable(glow::BLEND);
            gl.pop_debug_group();
        }
    }
}

fn load_brush_into_texture(gl: &glow::Context, brush: &Brush, texture: &glow::Texture) {
    unsafe {
        gl.active_texture(gl_utils::texture_unit_id_to_gl(0));
        gl.bind_texture(glow::TEXTURE_2D, Some(*texture));
        
        gl.object_label(
            glow::TEXTURE_2D,
            std::mem::transmute(*texture),
            Some(format!("Brush{}", brush.name)),
        );

        match &brush.bitmap {
            BrushGlyph::Png(data) => {
                let decoder = png::Decoder::new(data.as_slice());
                let (info, mut reader) = decoder.read_info().unwrap();
                // Allocate the output buffer.
                let mut buf = vec![0; info.buffer_size()];
                // Read the next frame. An APNG might contain multiple frames.
                reader.next_frame(&mut buf).unwrap();

                let tex_format = match reader.output_color_type() {
                    (ColorType::RGB, BitDepth::Eight) => gl_utils::TextureFormat::RGB8,
                    (ColorType::RGB, BitDepth::Sixteen) => gl_utils::TextureFormat::RGBA16UI,
                    (ColorType::RGBA, BitDepth::Eight) => gl_utils::TextureFormat::RGBA8,
                    (ColorType::RGBA, BitDepth::Sixteen) => gl_utils::TextureFormat::RGBA16UI,
                    (ColorType::Grayscale, BitDepth::Eight) => gl_utils::TextureFormat::R8,
                    (ColorType::Grayscale, BitDepth::Sixteen) => gl_utils::TextureFormat::R16UI,
                    (_, _) => unimplemented!("Unsupported PNG Pixel Type"),
                };

                let levels = (info.width as f32).log2().ceil() as i32;

                gl.tex_storage_2d(
                    glow::TEXTURE_2D,
                    levels,
                    tex_format.to_sized_internal_format(),
                    info.width as i32,
                    info.height as i32,
                );

                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::LINEAR as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::LINEAR_MIPMAP_LINEAR as i32,
                );
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

                gl.tex_sub_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    0,
                    0,
                    info.width as i32,
                    info.height as i32,
                    tex_format.to_format(),
                    tex_format.to_type(),
                    glow::PixelUnpackData::Slice(&buf),
                );
                if levels > 1 {
                    gl.generate_mipmap(glow::TEXTURE_2D);
                }
            }
        }
    }
}
