use crate::opengl::err::Result;
use gl;
use std::mem::size_of;
use std::ptr::null;
use std::rc::Rc;

pub struct VertexBuffer {
    pub(crate) buffer: Rc<Buffer>,
    /// The number of bytes that can be stored in this buffer.
    pub(crate) capacity: usize,
    /// The number of vertices that can be stored in this buffer.
    pub(crate) count: usize,
}

impl VertexBuffer {
    pub(crate) fn with_capacity<T>(count: usize) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        unsafe {
            let capacity = count * size_of::<T>();

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
                count,
            })
        }
    }

    pub(crate) fn with_data<T>(data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        unsafe {
            let count = data.len();
            let capacity = count * size_of::<T>();

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
                count,
            })
        }
    }

    pub(crate) fn update_with_slice<T>(&mut self, data: &[T]) -> Result<()>
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

            self.count = data.len();

            Ok(())
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub(crate) struct Buffer(pub(crate) u32);

impl<'a> Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}
