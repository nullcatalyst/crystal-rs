use crate::opengl::err::Result;
use crate::opengl::internal::Buffer;
use gl;
use std::mem::size_of;
use std::ptr::null;
use std::rc::Rc;

pub struct VertexBuffer {
    pub(crate) buffer: Rc<Buffer>,
    /// The number of bytes that can be stored in this buffer.
    pub(crate) capacity: usize,
}

impl VertexBuffer {
    pub(crate) fn with_capacity(capacity: usize) -> Result<VertexBuffer> {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                capacity as isize,
                null(),
                gl::DYNAMIC_DRAW,
            );

            Ok(VertexBuffer {
                buffer: Rc::from(Buffer(buffer)),
                capacity,
            })
        }
    }

    pub(crate) fn with_data<T>(data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        unsafe {
            let capacity = data.len() * size_of::<T>();

            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                capacity as isize,
                data.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            Ok(VertexBuffer {
                buffer: Rc::from(Buffer(buffer)),
                capacity,
            })
        }
    }

    pub(crate) fn update<T>(&mut self, data: &[T]) -> Result<()>
    where
        T: Sized,
    {
        unsafe {
            let length = data.len() * size_of::<T>();
            if length > self.capacity {
                return Err(
                    "updating vertex buffer: new data is longer than buffer capacity".into(),
                );
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer.0);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                length as isize,
                data.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            Ok(())
        }
    }
}
