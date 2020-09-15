use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub(crate) struct Buffer {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) buffer: WebGlBuffer,
}

impl<'a> Drop for Buffer {
    fn drop(&mut self) {
        self.context.delete_buffer(Some(&self.buffer));
    }
}
