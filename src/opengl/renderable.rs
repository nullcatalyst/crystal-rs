use crate::opengl::index_buffer::IndexBuffer;
use crate::opengl::vertex_buffer::VertexBuffer;
use gl;

pub struct Binding<'a> {
    pub attribute: u32,
    pub buffer: &'a VertexBuffer,
    pub offset: usize,
    pub stride: usize,
    pub instanced: bool,
}

pub struct Renderable {
    pub(crate) vertex_array: u32,
}

impl<'a> Drop for Renderable {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array);
        }
    }
}

impl Renderable {
    pub(crate) fn from_bindings(bindings: &[Binding]) -> Result<Renderable, String> {
        unsafe {
            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);

            for binding in bindings {
                gl::BindBuffer(gl::ARRAY_BUFFER, binding.buffer.buffer);
                gl::VertexAttribPointer(
                    binding.attribute,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    binding.stride as i32,
                    binding.offset as usize as *const _,
                );
                gl::VertexAttribDivisor(binding.attribute, if binding.instanced { 1 } else { 0 });
                gl::EnableVertexAttribArray(binding.attribute);
            }

            Ok(Renderable { vertex_array })
        }
    }

    pub(crate) fn from_bindings_and_index(
        bindings: &[Binding],
        index_buffer: &IndexBuffer,
    ) -> Result<Renderable, String> {
        unsafe {
            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);

            for binding in bindings {
                gl::BindBuffer(gl::ARRAY_BUFFER, binding.buffer.buffer);
                gl::VertexAttribPointer(
                    binding.attribute,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    binding.stride as i32,
                    binding.offset as usize as *const _,
                );
                gl::VertexAttribDivisor(binding.attribute, if binding.instanced { 1 } else { 0 });
                gl::EnableVertexAttribArray(binding.attribute);
            }

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.buffer);

            Ok(Renderable { vertex_array })
        }
    }
}
