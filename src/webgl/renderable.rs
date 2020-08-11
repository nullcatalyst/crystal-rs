use crate::webgl::{index_buffer::IndexBuffer, vertex_buffer::VertexBuffer};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

pub struct Binding<'a> {
    pub attribute: u32,
    pub buffer: &'a VertexBuffer,
    pub offset: usize,
    pub stride: usize,
    pub instanced: bool,
}

pub struct Renderable {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) vertex_array: WebGlVertexArrayObject,
}

impl<'a> Drop for Renderable {
    fn drop(&mut self) {
        self.context.delete_vertex_array(Some(&self.vertex_array));
    }
}

impl Renderable {
    pub(crate) fn from_bindings(
        gl: &Rc<WebGl2RenderingContext>,
        bindings: &[Binding],
    ) -> Result<Renderable, JsValue> {
        if let Some(vertex_array) = gl.create_vertex_array() {
            gl.bind_vertex_array(Some(&vertex_array));

            for binding in bindings {
                gl.bind_buffer(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    Some(&binding.buffer.buffer),
                );
                gl.vertex_attrib_pointer_with_i32(
                    binding.attribute,
                    4,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    binding.stride as i32,
                    binding.offset as i32,
                );
                gl.vertex_attrib_divisor(binding.attribute, if binding.instanced { 1 } else { 0 });
                gl.enable_vertex_attrib_array(binding.attribute);
            }

            Ok(Renderable {
                context: Rc::clone(gl),
                vertex_array,
            })
        } else {
            Err("creating vertex array object".into())
        }
    }

    pub(crate) fn from_bindings_and_index(
        gl: &Rc<WebGl2RenderingContext>,
        bindings: &[Binding],
        index_buffer: &IndexBuffer,
    ) -> Result<Renderable, JsValue> {
        if let Some(vertex_array) = gl.create_vertex_array() {
            gl.bind_vertex_array(Some(&vertex_array));

            for binding in bindings {
                gl.bind_buffer(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    Some(&binding.buffer.buffer),
                );
                gl.vertex_attrib_pointer_with_i32(
                    binding.attribute,
                    4,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    binding.stride as i32,
                    binding.offset as i32,
                );
                gl.vertex_attrib_divisor(binding.attribute, if binding.instanced { 1 } else { 0 });
                gl.enable_vertex_attrib_array(binding.attribute);
            }

            gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&index_buffer.buffer),
            );

            Ok(Renderable {
                context: Rc::clone(gl),
                vertex_array,
            })
        } else {
            Err("creating vertex array object".into())
        }
    }
}
