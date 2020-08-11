use crate::opengl::*;
use gl;
use std::ffi::CString;
use std::mem::size_of;
use std::path::Path;
use std::ptr::null;

pub struct Context {}

impl Context {
    pub fn new() -> Result<Context, String> {
        Ok(Context {})
    }

    pub fn load_with<F>(load_function: F) -> Result<Context, String>
    where
        F: Fn(&str) -> *const std::ffi::c_void,
    {
        gl::load_with(load_function);

        Ok(Context {})
    }

    pub fn create_shader<'a>(
        &self,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Shader, String> {
        Shader::from_source(vertex_source, fragment_source)
    }

    pub fn create_state(&self, desc: &StateDesc) -> Result<State, String> {
        State::from_desc(desc)
    }

    pub fn create_texture<P>(&self, image_url: P) -> Result<Texture, String>
    where
        P: AsRef<Path>,
    {
        Texture::from_path(image_url, TextureFilter::Nearest)
    }

    pub fn create_texture_with_filter<P>(
        &self,
        image_url: P,
        filter: TextureFilter,
    ) -> Result<Texture, String>
    where
        P: AsRef<Path>,
    {
        Texture::from_path(image_url, filter)
    }

    pub fn create_uniform_buffer<T>(&self, data: &T) -> Result<UniformBuffer, String>
    where
        T: Sized,
    {
        UniformBuffer::from_value(data)
    }

    pub fn update_uniform_buffer<T>(
        &self,
        uniform_buffer: &mut UniformBuffer,
        data: &T,
    ) -> Result<(), String>
    where
        T: Sized,
    {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, uniform_buffer.buffer);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                size_of::<T>() as isize,
                data as *const T as *const _,
                gl::DYNAMIC_DRAW,
            );

            Ok(())
        }
    }

    pub fn create_vertex_buffer<T>(&self, data: &[T]) -> Result<VertexBuffer, String>
    where
        T: Sized,
    {
        VertexBuffer::from_slice(data)
    }

    pub fn update_vertex_buffer<T>(
        &self,
        vertex_buffer: &mut VertexBuffer,
        data: &[T],
    ) -> Result<(), String>
    where
        T: Sized,
    {
        vertex_buffer.update_with_slice(data)
    }

    pub fn create_index_buffer<T>(&self, data: &[u16]) -> Result<IndexBuffer, String> {
        IndexBuffer::from_slice(data)
    }

    pub fn create_renderable(&self, bindings: &[Binding]) -> Result<Renderable, String> {
        Renderable::from_bindings(bindings)
    }

    pub fn create_renderable_with_index(
        &self,
        bindings: &[Binding],
        index_buffer: &IndexBuffer,
    ) -> Result<Renderable, String> {
        Renderable::from_bindings_and_index(bindings, index_buffer)
    }

    pub fn get_uniform(&self, shader: &Shader, uniform_name: &str) -> Result<Location, String> {
        unsafe {
            let uniform_name_cstr = match CString::new(uniform_name) {
                Ok(uniform_name) => uniform_name,
                Err(..) => {
                    return Err("converting shader name to c string".into());
                }
            };

            let location = gl::GetUniformBlockIndex(shader.program, uniform_name_cstr.as_ptr());
            if location != gl::INVALID_INDEX {
                return Ok(location as i32);
            }

            let location = gl::GetUniformLocation(shader.program, uniform_name_cstr.as_ptr());
            if location >= 0 {
                return Ok(location);
            }

            Err(format!("shader uniform {} not found", uniform_name))
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn clear_with_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn use_shader(&self, shader: &Shader) {
        unsafe {
            gl::UseProgram(shader.program);
        }
    }

    pub fn use_shader_with_state(&self, shader: &Shader, state: &State) {
        unsafe {
            gl::UseProgram(shader.program);

            if state.depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            if state.depth_write {
                gl::DepthMask(gl::TRUE);
            } else {
                gl::DepthMask(gl::FALSE);
            }

            if state.alpha_blend {
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Enable(gl::BLEND);
            } else {
                gl::Disable(gl::BLEND);
            }

            let mut binding = 0;
            for uniform in state.uniforms.iter() {
                match uniform {
                    UniformInternal::Buffer(location, uniform_buffer_id) => {
                        gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, *uniform_buffer_id);
                        gl::UniformBlockBinding(shader.program, *location as u32, binding);
                    }
                    _ => {}
                }

                binding += 1;
            }
        }
    }

    pub fn draw(&self, renderable: &Renderable, vertex_count: usize, instance_count: usize) {
        unsafe {
            gl::BindVertexArray(renderable.vertex_array);
            gl::DrawArraysInstanced(
                gl::TRIANGLE_STRIP,
                0,
                vertex_count as i32,
                instance_count as i32,
            );
        }
    }

    pub fn draw_indexed(&self, renderable: &Renderable, index_count: usize, instance_count: usize) {
        unsafe {
            gl::BindVertexArray(renderable.vertex_array);
            gl::DrawElementsInstanced(
                gl::TRIANGLE_STRIP,
                index_count as i32,
                gl::UNSIGNED_SHORT,
                null(),
                instance_count as i32,
            );
        }
    }
}
