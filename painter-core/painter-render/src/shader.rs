use glow::{Context, HasContext, Program, FRAGMENT_SHADER, VERTEX_SHADER};

#[derive(Debug)]
pub enum ShaderError {
    ShaderAllocError(String),
    ShaderProgramAllocError(String),
    ShaderCompileError {
        shader_type: u32,
        compiler_output: String,
        shader_text: String,
    },
    ShaderLinkError(String),
}

pub struct SimpleShader {
    pub program: Program,
    pub attrib_vertex_positions: u32,
}

impl SimpleShader {
    pub fn new(gl: &Context, vert: &str, frag: &str, name: &str) -> Result<Self, ShaderError> {
        let program = unsafe { init_shader_program(gl, vert, frag, name)? };
        let attrib_vertex_positions = unsafe {
            gl.get_attrib_location(program, "aVertexPosition")
                .expect("No vertx positions?")
        };

        Ok(Self {
            program,
            attrib_vertex_positions,
        })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}

unsafe fn load_shader(
    gl: &Context,
    shader_type: u32,
    shader_text: &str,
) -> Result<glow::Shader, ShaderError> {
    let shader = gl
        .create_shader(shader_type)
        .map_err(ShaderError::ShaderAllocError)?;

    gl.shader_source(shader, shader_text);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        let compiler_output = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        return Err(ShaderError::ShaderCompileError {
            shader_type,
            compiler_output: compiler_output,
            shader_text: shader_text.to_string(),
        });
    }
    Ok(shader)
}

pub unsafe fn init_shader_program(
    gl: &Context,
    vert_source: &str,
    frag_source: &str,
    name: &str,
) -> Result<Program, ShaderError> {
    assert_eq!(gl.get_error(), glow::NO_ERROR);
    let vert_shader = load_shader(gl, VERTEX_SHADER, vert_source)?;
    let frag_shader = load_shader(gl, FRAGMENT_SHADER, frag_source)?;

    gl.object_label(
        glow::SHADER,
        std::mem::transmute(vert_shader),
        Some(&format!("{}Vert", name)),
    );
    gl.object_label(
        glow::SHADER,
        std::mem::transmute(frag_shader),
        Some(&format!("{}Frag", name)),
    );

    let shader_program = gl
        .create_program()
        .map_err(ShaderError::ShaderProgramAllocError)?;
    gl.object_label(
        glow::PROGRAM,
        std::mem::transmute(shader_program),
        Some(&format!("{}Program", name)),
    );
    gl.attach_shader(shader_program, vert_shader);
    gl.attach_shader(shader_program, frag_shader);

    gl.link_program(shader_program);

    if !(gl.get_program_link_status(shader_program)) {
        let compiler_output = gl.get_program_info_log(shader_program);
        gl.delete_program(shader_program);
        gl.delete_shader(vert_shader);
        gl.delete_shader(frag_shader);
        return Err(ShaderError::ShaderLinkError(compiler_output));
    }
    assert_eq!(gl.get_error(), glow::NO_ERROR);
    Ok(shader_program)
}
