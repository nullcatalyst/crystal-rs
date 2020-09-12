use gl;

pub(crate) struct Program(pub(crate) u32);

impl<'a> Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}
