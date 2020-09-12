use crate::opengl::err::Result;
use crate::opengl::internal::Buffer;
use crate::opengl::VertexBuffer;
use std::cell::Cell;
use std::rc::Rc;

pub struct Renderable {
    pub(crate) vertex_arrays: Cell<Vec<(u32, VertexArray)>>,
    pub(crate) vertex_buffers: Vec<(u32, Rc<Buffer>)>,
}

impl Renderable {
    pub(crate) fn new(vertex_buffers: &[(u32, &VertexBuffer)]) -> Result<Renderable> {
        Ok(Renderable {
            vertex_arrays: Cell::from(Vec::new()),
            vertex_buffers: vertex_buffers
                .iter()
                .map(|(i, vertex_buffer)| (*i, vertex_buffer.buffer.clone()))
                .collect(),
        })
    }
}

pub(crate) struct VertexArray(pub(crate) u32);

impl<'a> Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0);
        }
    }
}
