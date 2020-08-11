use gl;
use std::mem::size_of;

pub struct VertexBuffer {
    pub(crate) buffer: u32,
    pub(crate) count: usize,
}

impl<'a> Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer);
        }
    }
}

impl VertexBuffer {
    pub(crate) fn from_slice<T>(data: &[T]) -> Result<VertexBuffer, String>
    where
        T: Sized,
    {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            Ok(VertexBuffer {
                buffer,
                count: data.len(),
            })
        }
    }

    pub(crate) fn update_with_slice<T>(
        &mut self,
        gl: &Rc<WebGl2RenderingContext>,
        data: &[T],
    ) -> Result<(), JsValue>
    where
        T: Sized,
    {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer.buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            vertex_buffer.count = data.len();

            Ok(())
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
