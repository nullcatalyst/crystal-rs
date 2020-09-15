use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub(crate) struct Program {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) program: WebGlProgram,
}

impl<'a> Drop for Program {
    fn drop(&mut self) {
        self.context.delete_program(Some(&self.program));
    }
}
