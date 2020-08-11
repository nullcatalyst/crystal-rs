use std::mem::size_of;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub struct VertexBuffer {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) buffer: WebGlBuffer,
    pub(crate) count: usize,
}

impl<'a> Drop for VertexBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(Some(&self.buffer));
    }
}

impl VertexBuffer {
    pub(crate) fn from_slice<T>(
        gl: &Rc<WebGl2RenderingContext>,
        data: &[T],
    ) -> Result<VertexBuffer, JsValue>
    where
        T: Sized,
    {
        if let Some(buffer) = gl.create_buffer() {
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            gl.buffer_data_with_u8_array(
                WebGl2RenderingContext::ARRAY_BUFFER,
                unsafe {
                    std::mem::transmute(std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * size_of::<T>(),
                    ))
                },
                WebGl2RenderingContext::STATIC_DRAW,
            );
            Ok(VertexBuffer {
                context: Rc::clone(gl),
                buffer,
                count: data.len(),
            })
        } else {
            Err("creating vertex buffer".into())
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
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        gl.buffer_data_with_u8_array(
            WebGl2RenderingContext::ARRAY_BUFFER,
            unsafe {
                std::mem::transmute(std::slice::from_raw_parts(
                    data.as_ptr() as *const u8,
                    data.len() * size_of::<T>(),
                ))
            },
            WebGl2RenderingContext::STATIC_DRAW,
        );

        self.count = data.len();

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
