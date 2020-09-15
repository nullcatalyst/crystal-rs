use crate::webgl::err::Result;
use crate::webgl::internal::Program;
use std::rc::Rc;
use std::str;
use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};

pub struct Library {
    #[allow(dead_code)]
    pub(crate) library_path: String,
}

impl Library {
    pub(crate) fn new(library_path: &str) -> Result<Library> {
        Ok(Library {
            library_path: library_path.into(),
        })
    }
}

pub struct Shader {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) program: Rc<Program>,
}

impl Shader {
    pub(crate) fn new(
        gl: &Rc<WebGl2RenderingContext>,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Shader> {
        let vertex_shader =
            compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_source)?;
        let fragment_shader =
            compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_source)?;
        if let Some(program) = gl.create_program() {
            gl.attach_shader(&program, &vertex_shader);
            gl.attach_shader(&program, &fragment_shader);
            gl.link_program(&program);
            if let Some(log) = gl.get_program_info_log(&program) {
                if log.len() > 0 {
                    return Err(log.into());
                }
            }
            Ok(Shader {
                context: Rc::clone(gl),
                program: Rc::from(Program {
                    context: Rc::clone(gl),
                    program,
                }),
            })
        } else {
            Err("creating shader program".into())
        }
    }

    /// This is an OPENGL ONLY API, and is only needed for a subset of OpenGL
    /// versions.
    pub fn get_uniform_location(&self, uniform_name: &str) -> Result<u32> {
        let gl = &self.context;
        let location = gl.get_uniform_block_index(&self.program.program, uniform_name);

        if location != WebGl2RenderingContext::INVALID_INDEX {
            Ok(location)
        } else {
            Err(format!("shader uniform \"{}\" not found", uniform_name).into())
        }
    }

    /// This is an WEBGL ONLY API, and may only needed for a subset of WebGL
    /// versions.
    pub fn get_texture_location(&self, texture_name: &str) -> Result<WebGlUniformLocation> {
        if let Some(location) = self
            .context
            .get_uniform_location(&self.program.program, texture_name)
        {
            Ok(location)
        } else {
            Err(format!("shader texture \"{}\" not found", texture_name).into())
        }
    }
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    shader_source: &str,
) -> Result<WebGlShader> {
    if let Some(shader) = gl.create_shader(shader_type) {
        gl.shader_source(&shader, shader_source);
        gl.compile_shader(&shader);
        if let Some(log) = gl.get_shader_info_log(&shader) {
            if log.len() > 0 {
                return Err(log.into());
            }
        }

        Ok(shader)
    } else {
        Err("failed to create shader".into())
    }
}
