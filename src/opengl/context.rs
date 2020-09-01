use crate::opengl::err::Result;
use crate::opengl::*;
use crate::shared::PipelineDesc;
use gl;
use std::fs;
use std::path::PathBuf;

#[cfg(feature = "use-sdl2")]
use sdl2;

// #[cfg(feature = "use-winit")]
// use winit;
// #[cfg(feature = "use-winit")]
// use winit::platform::macos::WindowExtMacOS;

pub struct Context {
    pub(crate) shader_path_prefix: PathBuf,
    #[cfg(feature = "use-sdl2")]
    _sdl_gl: Option<sdl2::video::GLContext>,
}

impl Context {
    pub fn new() -> Result<Context> {
        Ok(Context {
            shader_path_prefix: PathBuf::new(),
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
            shader_path_prefix: PathBuf::new(),
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
            shader_path_prefix: PathBuf::new(),
            _sdl_gl: Some(sdl2_gl),
        })
    }

    pub fn next_frame(&mut self) -> Result<CommandBuffer> {
        CommandBuffer::new()
    }

    pub fn set_shader_library_location(&mut self, library_path: &str) -> Result<()> {
        self.shader_path_prefix = PathBuf::from(library_path);
        Ok(())
    }

    pub fn create_shader(&self, vertex_file: &str, fragment_file: &str) -> Result<Shader> {
        let vertex_source = fs::read_to_string(self.shader_path_prefix.join(vertex_file))
            .map_err(|e| e.to_string())?;
        let fragment_source = fs::read_to_string(self.shader_path_prefix.join(fragment_file))
            .map_err(|e| e.to_string())?;

        Shader::new(vertex_source.as_str(), fragment_source.as_str())
    }

    pub fn create_pipeline(&self, shader: &Shader, desc: &PipelineDesc) -> Result<Pipeline> {
        Pipeline::new(shader, desc)
    }

    pub fn create_vertex_buffer_with_capacity<T>(&self, count: usize) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        VertexBuffer::with_capacity::<T>(count)
    }

    pub fn create_vertex_buffer_with_data<T>(&self, data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        VertexBuffer::with_data(data)
    }

    pub fn update_vertex_buffer<T>(
        &self,
        vertex_buffer: &mut VertexBuffer,
        data: &[T],
    ) -> Result<()>
    where
        T: Sized,
    {
        vertex_buffer.update_with_slice(data)
    }

    pub fn create_renderable(&self, vertex_buffers: &[(u32, &VertexBuffer)]) -> Result<Renderable> {
        Renderable::new(vertex_buffers)
    }
}
