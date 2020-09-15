use crate::shared::PipelineDesc;
use crate::webgl::err::Result;
use crate::webgl::*;
use crate::TextureFilter;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlElement, WebGl2RenderingContext, WebGlContextAttributes};

pub struct Context {
    context: Rc<WebGl2RenderingContext>,
}

impl Context {
    pub fn with_context(context: WebGl2RenderingContext) -> Result<Context> {
        Ok(Context {
            context: Rc::from(context),
        })
    }

    pub fn with_canvas(canvas: HtmlCanvasElement) -> Result<Context> {
        let mut context_attributes = WebGlContextAttributes::new();
        context_attributes
            .alpha(false)
            .antialias(true)
            .depth(true)
            .preserve_drawing_buffer(true);
        // let context_attributes = {
        //     let obj = Object::new();
        //     Reflect::set(&obj, &"alpha".into(), &false.into())?;
        //     Reflect::set(&obj, &"desynchronized".into(), &true.into())?;
        //     Reflect::set(&obj, &"antialias".into(), &true.into())?;
        //     Reflect::set(&obj, &"depth".into(), &true.into())?;
        //     Reflect::set(&obj, &"preserveDrawingBuffer".into(), &true.into())?;
        //     obj
        // };

        let context = canvas
            .get_context_with_context_options("webgl2", &context_attributes)?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        Context::with_context(context)
    }

    pub fn next_frame(&mut self) -> Result<CommandBuffer> {
        CommandBuffer::new(&self.context)
    }

    pub fn create_library(&mut self, library_path: &str) -> Result<Library> {
        Library::new(library_path)
    }

    pub fn create_shader(
        &mut self,
        _library: &Library,
        vertex_file: &str,
        fragment_file: &str,
    ) -> Result<Shader> {
        let vertex_source = element_contents_from_id(vertex_file)?;
        let fragment_source = element_contents_from_id(fragment_file)?;

        Shader::new(
            &self.context,
            vertex_source.as_str(),
            fragment_source.as_str(),
        )
    }

    pub fn create_pipeline(&mut self, shader: &Shader, desc: &PipelineDesc) -> Result<Pipeline> {
        Pipeline::new(shader, desc)
    }

    pub fn create_texture(&mut self, image_path: &str) -> Result<Texture> {
        Texture::new(&self.context, image_path, TextureFilter::Nearest)
    }

    pub fn create_uniform_buffer_with_capacity(
        &mut self,
        capacity: usize,
    ) -> Result<UniformBuffer> {
        UniformBuffer::with_capacity(&self.context, capacity)
    }

    pub fn create_uniform_buffer_with_value<T>(&mut self, value: &T) -> Result<UniformBuffer>
    where
        T: Sized,
    {
        UniformBuffer::with_data(&self.context, value)
    }

    pub fn update_uniform_buffer<T>(
        &mut self,
        uniform_buffer: &mut UniformBuffer,
        data: &T,
    ) -> Result<()>
    where
        T: Sized,
    {
        uniform_buffer.update(data)
    }

    pub fn create_texture_with_filter(
        &mut self,
        image_path: &str,
        filter: TextureFilter,
    ) -> Result<Texture> {
        Texture::new(&self.context, image_path, filter)
    }

    pub fn create_vertex_buffer_with_capacity(&mut self, capacity: usize) -> Result<VertexBuffer> {
        VertexBuffer::with_capacity(&self.context, capacity)
    }

    pub fn create_vertex_buffer_with_data<T>(&self, data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        VertexBuffer::with_data(&self.context, data)
    }

    pub fn update_vertex_buffer<T>(
        &mut self,
        vertex_buffer: &mut VertexBuffer,
        data: &[T],
    ) -> Result<()>
    where
        T: Sized,
    {
        vertex_buffer.update(data)
    }

    pub fn create_renderable(
        &mut self,
        vertex_buffers: &[(u32, &VertexBuffer)],
    ) -> Result<Renderable> {
        Renderable::new(vertex_buffers)
    }
}

fn element_contents_from_id(id: &str) -> Result<String> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let contents = document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    match contents.text_content() {
        Some(contents) => Ok(contents),
        None => Err(format!(
            "reading HTML element text content for element with id=\"{}\"",
            id
        )
        .into()),
    }
}
