use crate::metal::err::Result;
use cocoa::foundation::NSRange;
use metal;
use std::mem::size_of;
use std::ptr::null;

pub struct VertexBuffer {
    pub(crate) buffer: metal::Buffer,
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

    pub(crate) fn with_data<T>(device: &metal::Device, data: &T) -> Result<UniformBuffer>
    where
        T: Sized,
    {
        let capacity = size_of::<T>();
        let buffer = device.new_buffer_with_data(
            data.as_ptr() as *const _,
            capacity as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache
            // metal::MTLResourceOptions::CPUCacheModeWriteCombined
                | metal::MTLResourceOptions::StorageModeManaged,
        );

        Ok(UniformBuffer { buffer, capacity })
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
