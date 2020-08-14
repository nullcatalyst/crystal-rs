use crate::opengl::err::CrystalResult;
use gl;
use std::mem::size_of;

pub struct UniformBuffer {
    pub(crate) buffer: u32,
}

impl<'a> Drop for UniformBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer);
        }
    }
}

impl UniformBuffer {
    pub(crate) fn from_value<T>(data: &T) -> CrystalResult<UniformBuffer>
    where
        T: Sized,
    {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                size_of::<T>() as isize,
                data as *const T as *const _,
                gl::DYNAMIC_DRAW,
            );

            Ok(UniformBuffer { buffer })
        }
    }
}
