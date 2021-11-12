use glow::HasContext;

/// Because we are working inside a GL state defined by GTK4 we have to be able
/// to set certain things back to how they should be before we do our final drawing
/// Ie: we have to be able to draw on the framebuffer that GTK4 is expecting is to
/// draw onto.
pub struct FrameBufferState {
    framebuffer: glow::NativeFramebuffer,
    pub resolution: [i32; 4],
}

impl FrameBufferState {
    pub fn from_current_gl_state(gl: &glow::Context) -> FrameBufferState {
        let framebuffer: glow::NativeFramebuffer = unsafe {
            let as_u32 = gl.get_parameter_i32(glow::FRAMEBUFFER_BINDING) as u32;
            std::mem::transmute::<u32, glow::NativeFramebuffer>(as_u32)
        };
        let mut resolution = [0,0,0,0];
        unsafe {
            gl.get_parameter_i32_slice(glow::VIEWPORT, &mut resolution);
        }
    
        FrameBufferState {
            framebuffer,
            resolution
        }
    }

    pub fn apply(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer)); 
            gl.viewport(self.resolution[0], self.resolution[1], self.resolution[2], self.resolution[3]);
        }
    } 
}