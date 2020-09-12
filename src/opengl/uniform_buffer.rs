use crate::opengl::err::Result;
use crate::opengl::internal::Buffer;
use gl;
use std::mem::size_of;
use std::ptr::null;
use std::rc::Rc;

pub struct UniformBuffer {
    pub(crate) buffer: Rc<Buffer>,
    /// The number of bytes that can be stored in this buffer.
    pub(crate) capacity: usize,
}

impl UniformBuffer {
    pub(crate) fn with_capacity(capacity: usize) -> Result<UniformBuffer> {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                capacity as isize,
                null(),
                gl::DYNAMIC_DRAW,
            );

            Ok(UniformBuffer {
                buffer: Rc::from(Buffer(buffer)),
                capacity,
            })
        }
    }

    pub(crate) fn with_data<T>(data: &T) -> Result<UniformBuffer>
    where
        T: Sized,
    {
        unsafe {
            let capacity = size_of::<T>();

            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                capacity as isize,
                data as *const T as *const _,
                gl::DYNAMIC_DRAW,
            );

            Ok(UniformBuffer {
                buffer: Rc::from(Buffer(buffer)),
                capacity,
            })
        }
    }

    pub(crate) fn update<T>(&mut self, data: &T) -> Result<()>
    where
        T: Sized,
    {
        unsafe {
            let length = size_of::<T>();
            if length > self.capacity {
                return Err(
                    "updating uniform buffer: new data is longer than buffer capacity".into(),
                );
            }

            gl::BindBuffer(gl::UNIFORM_BUFFER, self.buffer.0);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                length as isize,
                data as *const T as *const _,
                gl::DYNAMIC_DRAW,
            );

            Ok(())
        }
    }
}
