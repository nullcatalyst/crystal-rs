use crate::webgl::*;
use js_sys::{Object, Reflect};
use std::mem::size_of;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub struct Context {
    context: Rc<WebGl2RenderingContext>,
}

impl Context {
    pub fn new(context: WebGl2RenderingContext) -> Result<Context, JsValue> {
        Ok(Context {
            context: Rc::from(context),
        })
    }

    pub fn from_canvas(canvas: HtmlCanvasElement) -> Result<Context, JsValue> {
        let context_options = {
            let obj = Object::new();
            Reflect::set(&obj, &"alpha".into(), &false.into())?;
            Reflect::set(&obj, &"desynchronized".into(), &true.into())?;
            Reflect::set(&obj, &"antialias".into(), &true.into())?;
            Reflect::set(&obj, &"depth".into(), &true.into())?;
            Reflect::set(&obj, &"preserveDrawingBuffer".into(), &true.into())?;
            obj
        };

        let context = canvas
            .get_context_with_context_options("webgl2", &context_options)?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        Context::new(context)
    }

    pub fn create_shader<'a>(
        &self,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Shader, JsValue> {
        Shader::from_source(&self.context, vertex_source, fragment_source)
    }

    pub fn create_state(&self, desc: &StateDesc) -> Result<State, JsValue> {
        State::from_desc(desc)
    }

    pub fn create_texture(&self, image_url: &str) -> Result<Texture, JsValue> {
        Texture::from_url(&self.context, image_url)
    }

    pub fn create_uniform_buffer<T>(&self, data: &T) -> Result<UniformBuffer, JsValue>
    where
        T: Sized,
    {
        UniformBuffer::from_value(&self.context, data)
    }

    pub fn update_uniform_buffer<T>(
        &self,
        uniform_buffer: &UniformBuffer,
        data: &T,
    ) -> Result<(), String>
    where
        T: Sized,
    {
        self.context.bind_buffer(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            Some(&uniform_buffer.buffer),
        );
        self.context.buffer_data_with_u8_array(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            unsafe {
                std::mem::transmute(std::slice::from_raw_parts(
                    data as *const T as *const _,
                    size_of::<T>(),
                ))
            },
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );

        Ok(())
    }

    pub fn create_vertex_buffer<T>(&self, data: &[T]) -> Result<VertexBuffer, JsValue>
    where
        T: Sized,
    {
        VertexBuffer::from_slice(&self.context, data)
    }

    pub fn update_vertex_buffer<T>(
        &self,
        vertex_buffer: &mut VertexBuffer,
        data: &[T],
    ) -> Result<(), JsValue>
    where
        T: Sized,
    {
        vertex_buffer.update_with_slice(&self.context, data)
    }

    pub fn create_index_buffer<T>(&self, data: &[u16]) -> Result<IndexBuffer, JsValue> {
        IndexBuffer::from_slice(&self.context, data)
    }

    pub fn create_renderable(&self, bindings: &[Binding]) -> Result<Renderable, JsValue> {
        Renderable::from_bindings(&self.context, bindings)
    }

    pub fn create_renderable_with_index(
        &self,
        bindings: &[Binding],
        index_buffer: &IndexBuffer,
    ) -> Result<Renderable, JsValue> {
        Renderable::from_bindings_and_index(&self.context, bindings, index_buffer)
    }

    pub fn get_uniform(&self, shader: &Shader, uniform_name: &str) -> Result<Location, String> {
        let location = self
            .context
            .get_uniform_block_index(&shader.program, uniform_name);
        if location != WebGl2RenderingContext::INVALID_INDEX {
            return Ok(Location::Buffer(location));
        }

        if let Some(location) = self
            .context
            .get_uniform_location(&shader.program, uniform_name)
        {
            return Ok(Location::Texture(location));
        }

        Err(format!(
            "shader uniform \"{}\" not found {}",
            uniform_name,
            self.context.get_error()
        ))
    }

    pub fn clear(&self) {
        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }

    pub fn clear_with_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }

    pub fn use_shader(&self, shader: &Shader) {
        self.context.use_program(Some(&shader.program));
    }

    pub fn use_shader_with_state(&self, shader: &Shader, state: &State) {
        self.context.use_program(Some(&shader.program));

        if state.depth_test {
            self.context.enable(WebGl2RenderingContext::DEPTH_TEST);
        } else {
            self.context.disable(WebGl2RenderingContext::DEPTH_TEST);
        }

        if state.depth_write {
            self.context.depth_mask(true);
        } else {
            self.context.depth_mask(false);
        }

        if state.alpha_blend {
            self.context.blend_func(
                WebGl2RenderingContext::SRC_ALPHA,
                WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
            );
            self.context.enable(WebGl2RenderingContext::BLEND);
        } else {
            self.context.disable(WebGl2RenderingContext::BLEND);
        }

        let mut binding = 0;
        for uniform in state.uniforms.iter() {
            match uniform {
                UniformInternal::Buffer(location, uniform_buffer_id) => {
                    self.context.bind_buffer_base(
                        WebGl2RenderingContext::UNIFORM_BUFFER,
                        binding,
                        Some(&uniform_buffer_id),
                    );
                    self.context.uniform_block_binding(
                        &shader.program,
                        match location {
                            Location::Buffer(location) => *location,
                            _ => panic!(),
                        },
                        binding,
                    );
                }
                _ => {}
            }

            binding += 1;
        }
    }

    pub fn draw(&self, renderable: &Renderable, vertex_count: usize, instance_count: usize) {
        self.context
            .bind_vertex_array(Some(&renderable.vertex_array));
        self.context.draw_arrays_instanced(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0,
            vertex_count as i32,
            instance_count as i32,
        );
    }

    pub fn draw_indexed(&self, renderable: &Renderable, index_count: usize, instance_count: usize) {
        self.context
            .bind_vertex_array(Some(&renderable.vertex_array));
        self.context.draw_elements_instanced_with_i32(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            index_count as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
            instance_count as i32,
        );
    }
}
