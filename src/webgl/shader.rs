use crate::webgl::err::CrystalResult;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct Shader {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) program: WebGlProgram,
}

impl<'a> Drop for Shader {
    fn drop(&mut self) {
        self.context.delete_program(Some(&self.program));
    }
}

impl Shader {
    pub(crate) fn from_source(
        gl: &Rc<WebGl2RenderingContext>,
        vertex_source: &str,
        fragment_source: &str,
    ) -> CrystalResult<Shader> {
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
                program,
            })
        } else {
            Err("failed to create shader program".into())
        }
    }
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    shader_source: &str,
) -> CrystalResult<WebGlShader> {
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
