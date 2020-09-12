use crate::opengl::err::Result;
use crate::opengl::internal::Program;
use gl;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::rc::Rc;
use std::str;

pub struct Library {
    pub(crate) library_path: PathBuf,
}

impl Library {
    pub(crate) fn new(library_path: &str) -> Result<Library> {
        Ok(Library {
            library_path: PathBuf::from(library_path),
        })
    }
}

pub struct Shader {
    pub(crate) program: Rc<Program>,
}

impl Shader {
    pub(crate) fn new(vertex_source: &str, fragment_source: &str) -> Result<Shader> {
        let vertex_shader = compile_shader(gl::VERTEX_SHADER, vertex_source)?;
        let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, fragment_source)?;

        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut link_status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_status);
            if link_status == 0 {
                let mut info_log_length = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut info_log_length);

                // Info log length includes the null terminator, so 1 means that the info log is an
                // empty string.
                let message = if info_log_length > 1 {
                    let mut info_log_buffer = Vec::with_capacity(info_log_length as usize);
                    info_log_buffer.set_len(info_log_length as usize);
                    gl::GetProgramInfoLog(
                        program,
                        info_log_length,
                        null_mut(),
                        info_log_buffer.as_mut_ptr() as *mut _,
                    );
                    let info_log = match str::from_utf8(&info_log_buffer) {
                        Ok(info_log) => info_log,
                        Err(_) => "<empty log message>",
                    };

                    format!("linking shader program: {}", info_log)
                } else {
                    format!("linking shader program: <empty log message>")
                };

                gl::DeleteProgram(program);
                return Err(message);
            }

            Ok(Shader {
                program: Rc::from(Program(program)),
            })
        }
    }

    /// This is an OPENGL ONLY API, and is only needed for a subset of OpenGL versions.
    pub fn get_uniform_location(&self, uniform_name: &str) -> Result<u32> {
        let location = unsafe {
            let uniform_name_cstr = match CString::new(uniform_name) {
                Ok(uniform_name) => uniform_name,
                Err(..) => {
                    return Err("converting uniform name to C string".into());
                }
            };

            gl::GetUniformBlockIndex(self.program.0, uniform_name_cstr.as_ptr())
        };

        if location != gl::INVALID_INDEX {
            Ok(location)
        } else {
            Err(format!("shader uniform \"{}\" not found", uniform_name))
        }
    }

    /// This is an OPENGL ONLY API, and is only needed for a subset of OpenGL versions.
    pub fn get_texture_location(&self, texture_name: &str) -> Result<i32> {
        let location = unsafe {
            let texture_name_cstr = match CString::new(texture_name) {
                Ok(texture_name) => texture_name,
                Err(..) => {
                    return Err("converting texture name to C string".into());
                }
            };

            gl::GetUniformLocation(self.program.0, texture_name_cstr.as_ptr())
        };

        if location >= 0 {
            Ok(location)
        } else {
            Err(format!("shader texture \"{}\" not found", texture_name))
        }
    }
}

fn compile_shader(shader_type: u32, shader_source: &str) -> Result<u32> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(
            shader,
            1,
            &(shader_source.as_ptr() as *const i8) as *const *const _,
            &(shader_source.len() as i32),
        );
        gl::CompileShader(shader);

        let mut compile_result = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compile_result);

        if compile_result == 0 {
            let mut info_log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);

            let shader_type = match shader_type {
                gl::VERTEX_SHADER => "vertex",
                gl::FRAGMENT_SHADER => "fragment",
                gl::COMPUTE_SHADER => "compute",
                _ => "unknown",
            };

            // Info log length includes the null terminator, so 1 means that the info log is an
            // empty string.
            let message = if info_log_length > 1 {
                let mut info_log_buffer = Vec::with_capacity(info_log_length as usize);
                info_log_buffer.set_len(info_log_length as usize);
                gl::GetShaderInfoLog(
                    shader,
                    info_log_length,
                    null_mut(),
                    info_log_buffer.as_mut_ptr() as *mut _,
                );
                let info_log = match str::from_utf8(&info_log_buffer) {
                    Ok(info_log) => info_log,
                    Err(_) => "<empty log message>",
                };
                format!("compiling {} shader: {}", shader_type, info_log)
            } else {
                format!("compiling {} shader: <empty log message>", shader_type)
            };
            gl::DeleteShader(shader);

            return Err(message);
        }

        Ok(shader)
    }
}
