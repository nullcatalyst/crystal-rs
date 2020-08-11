use std::mem::size_of;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub struct IndexBuffer {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) buffer: WebGlBuffer,
    pub(crate) count: usize,
}

impl<'a> Drop for IndexBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(Some(&self.buffer));
    }
}

impl IndexBuffer {
    pub(crate) fn from_slice(
        gl: &Rc<WebGl2RenderingContext>,
        data: &[u16],
    ) -> Result<IndexBuffer, JsValue> {
        if let Some(buffer) = gl.create_buffer() {
            gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
            gl.buffer_data_with_u8_array(
                WebGl2RenderingContext::ARRAY_BUFFER,
                unsafe {
                    std::mem::transmute(std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * size_of::<u16>(),
                    ))
                },
                WebGl2RenderingContext::STATIC_DRAW,
            );
            Ok(IndexBuffer {
                context: Rc::clone(gl),
                buffer,
                count: data.len(),
            })
        } else {
            Err("failed to create index buffer".into())
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
