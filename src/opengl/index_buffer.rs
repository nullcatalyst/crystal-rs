use crate::opengl::err::CrystalResult;
use gl;
use std::mem::size_of;

pub struct IndexBuffer {
    pub(crate) buffer: u32,
    pub(crate) count: usize,
}

impl<'a> Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer);
        }
    }
}

impl IndexBuffer {
    pub(crate) fn from_slice(data: &[u16]) -> CrystalResult<IndexBuffer> {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * size_of::<u16>()) as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            Ok(IndexBuffer {
                buffer,
                count: data.len(),
            })
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
