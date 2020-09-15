use crate::webgl::err::Result;
use crate::webgl::internal::Buffer;
use crate::webgl::VertexBuffer;
use std::cell::Cell;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

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

pub(crate) struct VertexArray {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) vertex_array: WebGlVertexArrayObject,
}

impl<'a> Drop for VertexArray {
    fn drop(&mut self) {
        self.context.delete_vertex_array(Some(&self.vertex_array));
    }
}
