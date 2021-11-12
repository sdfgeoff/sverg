use glow::HasContext;
use std::convert::TryInto;
use log::info;

use super::gl_utils::{color_attachment_int_to_gl, TextureFormat};

pub struct Canvas {
    framebuffer: glow::Framebuffer,
    resolution: [u32; 2],
    pub texture: glow::Texture,
}

#[derive(Debug)]
pub enum CanvasError {
    CreateFrameBufferFailed(String),
    CreateTextureFailed(String),
}

impl Canvas {
    pub fn new(gl: &glow::Context, resolution: [u32; 2], name: &str) -> Result<Self, CanvasError> {
        let framebuffer = unsafe {
            gl.create_framebuffer()
                .map_err(CanvasError::CreateFrameBufferFailed)?
        };
        unsafe {
            gl.object_label(
                glow::FRAMEBUFFER,
                std::mem::transmute(framebuffer),
                Some(name),
            )
        };

        // Set it up so we are operating on our framebuffer and have a texture unit to play with
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
            gl.active_texture(glow::TEXTURE0);
            assert_eq!(gl.get_error(), glow::NO_ERROR);
        }

        let texture = create_canvas_texture(gl)?;
        unsafe {
            gl.object_label(
                glow::TEXTURE,
                std::mem::transmute(texture),
                Some(&format!("{}Texture", name)),
            )
        };
        let attachment = color_attachment_int_to_gl(0);

        unsafe {
            // let levels = { (resolution[0] as f32).log2().ceil() as i32 };
            let format = TextureFormat::RGBA32F;

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            // For textures that can change size we use TexImage2d
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                format.to_sized_internal_format() as i32,
                resolution[0].try_into().unwrap(),
                resolution[1].try_into().unwrap(),
                0,
                format.to_format(), // If we were passing in an existing image into data, this would be meaningful
                format.to_type(), // If we were passing in an existing image into data, this would be meaningful
                None, // but we are passing in None here, so the above two values are ignored.
            );
            gl.generate_mipmap(glow::TEXTURE_2D);

            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                attachment,
                glow::TEXTURE_2D,
                Some(texture),
                0,
            );

            gl.draw_buffers(&vec![attachment]);
        }
        Ok(Self {
            framebuffer,
            texture,
            resolution,
        })
    }

    pub fn resize(&mut self, gl: &glow::Context, resolution: [u32; 2]) {
        if resolution != self.resolution {
            self.resolution = resolution;
            unsafe {
                info!("resizing_canvas");
                let format = TextureFormat::RGBA32F;

                gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
                // For textures that can change size we use TexImage2d
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    format.to_sized_internal_format() as i32,
                    self.resolution[0].try_into().unwrap(),
                    self.resolution[1].try_into().unwrap(),
                    0,
                    format.to_format(), // If we were passing in an existing image into data, this would be meaningful
                    format.to_type(), // If we were passing in an existing image into data, this would be meaningful
                    None, // but we are passing in None here, so the above two values are ignored.
                );
                gl.generate_mipmap(glow::TEXTURE_2D);
            }
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.resolution[0] as f32) / (self.resolution[1] as f32)
    }

    pub fn make_active(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            gl.viewport(0, 0, self.resolution[0] as i32, self.resolution[1] as i32);
        }
    }
}

/// Create the texture and set it up
/// Does not allocate storage for the texture.
pub fn create_canvas_texture(gl: &glow::Context) -> Result<glow::Texture, CanvasError> {
    unsafe {
        assert_eq!(gl.get_error(), glow::NO_ERROR);
    }

    let new_tex = unsafe {
        gl.create_texture()
            .map_err(CanvasError::CreateTextureFailed)?
    };

    unsafe {
        gl.bind_texture(glow::TEXTURE_2D, Some(new_tex));

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
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );

        assert_eq!(gl.get_error(), glow::NO_ERROR);
    }

    Ok(new_tex)
}
