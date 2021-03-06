use crate::metal::err::Result;
use cocoa::foundation::NSRange;
use foreign_types::ForeignType;
use metal;
use std::mem::size_of;
use std::ptr::null_mut;

pub struct VertexBuffer {
    pub(crate) buffer: metal::Buffer,
    /// The number of bytes that can be stored in this buffer.
    pub(crate) capacity: usize,
}

impl VertexBuffer {
    pub(crate) fn with_capacity(device: &metal::Device, capacity: usize) -> Result<VertexBuffer> {
        let buffer = device.new_buffer(
            capacity as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache
                | metal::MTLResourceOptions::StorageModeManaged,
        );

        Ok(VertexBuffer { buffer, capacity })
    }

    pub(crate) fn with_data<T>(device: &metal::Device, data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        let capacity = data.len() * size_of::<T>();
        let buffer = device.new_buffer_with_data(
            data.as_ptr() as *const _,
            capacity as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache
                | metal::MTLResourceOptions::StorageModeManaged,
        );

        Ok(VertexBuffer { buffer, capacity })
    }

    pub(crate) fn update<T>(&mut self, data: &[T]) -> Result<()>
    where
        T: Sized,
    {
        let length = data.len() * size_of::<T>();
        if length > self.capacity {
            return Err("updating vertex buffer: new data is longer than buffer capacity".into());
        }

        let p = self.buffer.contents();
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), p as *mut _, data.len());
        }

        self.buffer
            .did_modify_range(NSRange::new(0 as u64, length as u64));

        Ok(())
    }
}

impl Default for VertexBuffer {
    fn default() -> VertexBuffer {
        VertexBuffer {
            buffer: unsafe { metal::Buffer::from_ptr(null_mut()) },
            capacity: 0,
        }
    }
}
