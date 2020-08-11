use std::mem::size_of;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[allow(dead_code)]
pub struct UniformBuffer {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) buffer: WebGlBuffer,
}

impl<'a> Drop for UniformBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(Some(&self.buffer));
    }
}

impl UniformBuffer {
    pub(crate) fn from_value<T>(
        gl: &Rc<WebGl2RenderingContext>,
        data: &T,
    ) -> Result<UniformBuffer, JsValue>
    where
        T: Sized,
    {
        if let Some(buffer) = gl.create_buffer() {
            gl.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, Some(&buffer));
            gl.buffer_data_with_u8_array(
                WebGl2RenderingContext::UNIFORM_BUFFER,
                unsafe {
                    std::mem::transmute(std::slice::from_raw_parts(
                        data as *const T as *const _,
                        size_of::<T>(),
                    ))
                },
                WebGl2RenderingContext::DYNAMIC_DRAW,
            );

            Ok(UniformBuffer {
                context: Rc::clone(gl),
                buffer,
            })
        } else {
            Err("failed to create uniform buffer".into())
        }
    }
}
