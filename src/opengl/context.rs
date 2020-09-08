use crate::opengl::err::Result;
use crate::opengl::*;
use crate::shared::PipelineDesc;
use crate::TextureFilter;
use gl;
use std::fs;

#[cfg(feature = "use-sdl2")]
use sdl2;

// #[cfg(feature = "use-winit")]
// use winit;
// #[cfg(feature = "use-winit")]
// use winit::platform::macos::WindowExtMacOS;

pub struct Context {
    #[cfg(feature = "use-sdl2")]
    _sdl_gl: Option<sdl2::video::GLContext>,
}

impl Context {
    pub fn new() -> Result<Context> {
        Ok(Context {
            #[cfg(feature = "use-sdl2")]
            _sdl_gl: None,
        })
    }

    pub fn with_loader<F>(load_function: F) -> Result<Context>
    where
        F: Fn(&str) -> *const std::ffi::c_void,
    {
        gl::load_with(load_function);

        Ok(Context {
            #[cfg(feature = "use-sdl2")]
            _sdl_gl: None,
        })
    }

    #[cfg(feature = "use-sdl2")]
    pub fn with_sdl2_window(
        video_subsystem: &sdl2::VideoSubsystem,
        window: &sdl2::video::Window,
    ) -> Result<Context> {
        let sdl2_gl = window.gl_create_context()?;
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        Ok(Context {
            _sdl_gl: Some(sdl2_gl),
        })
    }

    pub fn next_frame(&mut self) -> Result<CommandBuffer> {
        CommandBuffer::new()
    }

    pub fn create_library(&mut self, library_path: &str) -> Result<Library> {
        Library::new(library_path)
    }

    pub fn create_shader(
        &mut self,
        library: &Library,
        vertex_file: &str,
        fragment_file: &str,
    ) -> Result<Shader> {
        let vertex_source = fs::read_to_string(library.library_path.join(vertex_file))
            .map_err(|e| e.to_string())?;
        let fragment_source = fs::read_to_string(library.library_path.join(fragment_file))
            .map_err(|e| e.to_string())?;

        Shader::new(vertex_source.as_str(), fragment_source.as_str())
    }

    pub fn create_pipeline(&mut self, shader: &Shader, desc: &PipelineDesc) -> Result<Pipeline> {
        Pipeline::new(shader, desc)
    }

    pub fn create_texture(&mut self, image_path: &str) -> Result<Texture> {
        Texture::new(image_path, TextureFilter::Nearest)
    }

    pub fn create_texture_with_filter(
        &mut self,
        image_path: &str,
        filter: TextureFilter,
    ) -> Result<Texture> {
        Texture::new(image_path, filter)
    }

    pub fn create_vertex_buffer_with_capacity(&mut self, capacity: usize) -> Result<VertexBuffer> {
        VertexBuffer::with_capacity(capacity)
    }

    pub fn create_vertex_buffer_with_data<T>(&self, data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        VertexBuffer::with_data(data)
    }

    pub fn update_vertex_buffer<T>(
        &mut self,
        vertex_buffer: &mut VertexBuffer,
        data: &[T],
    ) -> Result<()>
    where
        T: Sized,
    {
        vertex_buffer.update_with_slice(data)
    }

    pub fn create_renderable(
        &mut self,
        vertex_buffers: &[(u32, &VertexBuffer)],
    ) -> Result<Renderable> {
        Renderable::new(vertex_buffers)
    }
}
