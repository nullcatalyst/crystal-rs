use gl;

pub(crate) struct Buffer(pub(crate) u32);

impl<'a> Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}
