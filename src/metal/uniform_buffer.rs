use crate::metal::err::Result;
use cocoa::foundation::NSRange;
use foreign_types::ForeignType;
use metal;
use std::mem::size_of;
use std::ptr::null_mut;

pub struct UniformBuffer {
    pub(crate) buffer: metal::Buffer,
    /// The number of bytes that can be stored in this buffer.
    pub(crate) capacity: usize,
}

impl UniformBuffer {
    pub(crate) fn with_capacity(device: &metal::Device, capacity: usize) -> Result<UniformBuffer> {
        let buffer = device.new_buffer(
            capacity as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache
                | metal::MTLResourceOptions::StorageModeManaged,
        );

        Ok(UniformBuffer { buffer, capacity })
    }

    pub(crate) fn with_data<T>(device: &metal::Device, data: &T) -> Result<UniformBuffer>
    where
        T: Sized,
    {
        let capacity = size_of::<T>();
        let buffer = device.new_buffer_with_data(
            data as *const T as *const _,
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
        let length = size_of::<T>();
        if length > self.capacity {
            return Err("updating uniform buffer: new data is longer than buffer capacity".into());
        }

        let p = self.buffer.contents();
        unsafe {
            std::ptr::copy_nonoverlapping(data, p as *mut _, 1);
        }

        self.buffer
            .did_modify_range(NSRange::new(0 as u64, length as u64));

        Ok(())
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Default for UniformBuffer {
    fn default() -> UniformBuffer {
        UniformBuffer {
            buffer: unsafe { metal::Buffer::from_ptr(null_mut()) },
            capacity: 0,
        }
    }
}
