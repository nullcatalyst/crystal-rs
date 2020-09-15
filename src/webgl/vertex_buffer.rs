use crate::webgl::err::Result;
use crate::webgl::internal::Buffer;
use std::mem::size_of;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct VertexBuffer {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) buffer: Rc<Buffer>,
    /// The number of bytes that can be stored in this buffer.
    pub(crate) capacity: usize,
}

impl VertexBuffer {
    pub(crate) fn with_capacity(
        gl: &Rc<WebGl2RenderingContext>,
        capacity: usize,
    ) -> Result<VertexBuffer> {
        if let Some(buffer) = gl.create_buffer() {
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            gl.buffer_data_with_u8_array(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &[],
                WebGl2RenderingContext::DYNAMIC_DRAW,
            );

            Ok(VertexBuffer {
                context: Rc::clone(gl),
                buffer: Rc::from(Buffer {
                    context: Rc::clone(gl),
                    buffer,
                }),
                capacity,
            })
        } else {
            Err("creating vertex buffer".into())
        }
    }

    pub(crate) fn with_data<T>(gl: &Rc<WebGl2RenderingContext>, data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        if let Some(buffer) = gl.create_buffer() {
            let capacity = data.len() * size_of::<T>();

            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            gl.buffer_data_with_u8_array(
                WebGl2RenderingContext::ARRAY_BUFFER,
                unsafe {
                    std::mem::transmute(std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        capacity,
                    ))
                },
                WebGl2RenderingContext::DYNAMIC_DRAW,
            );

            Ok(VertexBuffer {
                context: Rc::clone(gl),
                buffer: Rc::from(Buffer {
                    context: Rc::clone(gl),
                    buffer,
                }),
                capacity,
            })
        } else {
            Err("creating vertex buffer".into())
        }
    }

    pub(crate) fn update<T>(&mut self, data: &[T]) -> Result<()>
    where
        T: Sized,
    {
        let length = data.len() * size_of::<T>();
        if length > self.capacity {
            return Err("updating vertex buffer: new data is longer than buffer capacity".into());
        }

        let gl = &self.context;
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.buffer.buffer),
        );
        gl.buffer_data_with_u8_array(
            WebGl2RenderingContext::ARRAY_BUFFER,
            unsafe {
                std::mem::transmute(std::slice::from_raw_parts(
                    data.as_ptr() as *const u8,
                    length,
                ))
            },
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );

        Ok(())
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
