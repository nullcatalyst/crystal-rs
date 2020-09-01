use crate::metal::err::Result;
use crate::metal::*;
use crate::PipelineDesc;
use cocoa::{appkit::NSView, appkit::NSWindow, base::id as cocoa_id};
use metal;
use objc::runtime::YES;

#[cfg(feature = "use-sdl2")]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
#[cfg(feature = "use-sdl2")]
use sdl2;

#[cfg(feature = "use-winit")]
use winit;
#[cfg(feature = "use-winit")]
use winit::platform::macos::WindowExtMacOS;

pub struct Context {
    pub(crate) device: metal::Device,
    pub(crate) layer: metal::CoreAnimationLayer,
    pub(crate) command_queue: metal::CommandQueue,
    /// The currently loaded library.
    pub(crate) library: Option<metal::Library>,
}

impl Context {
    pub fn new(device: metal::Device, layer: metal::CoreAnimationLayer) -> Result<Context> {
        let command_queue = device.new_command_queue();

        Ok(Context {
            device,
            layer,
            command_queue,
            library: None,
        })
    }

    #[cfg(feature = "use-sdl2")]
    pub fn with_sdl2_window(window: &sdl2::video::Window) -> Result<Context> {
        let device = if let Some(device) = metal::Device::system_default() {
            device
        } else {
            return Err("creating metal device".into());
        };

        let layer = metal::CoreAnimationLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            let raw_window = match window.raw_window_handle() {
                RawWindowHandle::MacOS(window) => window,
                _ => panic!("creating metal view for window: invalid window type"),
            };

            let view = (raw_window.ns_window as cocoa_id).contentView();
            // println!("metal view = {:?}", view);
            view.setWantsLayer(YES);
            view.setLayer(std::mem::transmute(layer.as_ref()));
        }

        let (width, height) = window.size();
        layer.set_drawable_size(metal::CGSize::new(width as f64, height as f64));

        Context::new(device, layer)
    }

    #[cfg(feature = "use-winit")]
    pub fn with_winit_window(window: &winit::window::Window) -> Result<Context> {
        let device = if let Some(device) = metal::Device::system_default() {
            device
        } else {
            return Err("creating metal device".into());
        };

        let layer = metal::CoreAnimationLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            let view = window.ns_view() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(std::mem::transmute(layer.as_ref()));
        }

        let draw_size = window.inner_size();
        layer.set_drawable_size(metal::CGSize::new(
            draw_size.width as f64,
            draw_size.height as f64,
        ));

        Context::new(device, layer)
    }

    pub fn next_frame(&mut self) -> Result<CommandBuffer> {
        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return Err("starting frame without drawable".into()),
        };

        CommandBuffer::new(drawable, self.command_queue.new_command_buffer())
    }

    pub fn set_shader_library_location(&mut self, library_path: &str) -> Result<()> {
        let library = self.device.new_library_with_file(library_path)?;
        self.library = Some(library);

        Ok(())
    }

    pub fn create_shader(&self, vertex_name: &str, fragment_name: &str) -> Result<Shader> {
        if let Some(library) = &self.library {
            Shader::new(library, vertex_name, fragment_name)
        } else {
            Err("creating shader: no metal library loaded".into())
        }
    }

    pub fn create_pipeline(&self, shader: &Shader, desc: &PipelineDesc) -> Result<Pipeline> {
        Pipeline::new(&self.device, shader, desc)
    }

    // pub fn create_texture<P>(&self, image_url: P) -> CrystalResult<Texture>
    // where
    //     P: AsRef<Path>,
    // {
    //     Texture::from_path(image_url, TextureFilter::Nearest)
    // }

    // pub fn create_texture_with_filter<P>(
    //     &self,
    //     image_url: P,
    //     filter: TextureFilter,
    // ) -> CrystalResult<Texture>
    // where
    //     P: AsRef<Path>,
    // {
    //     Texture::from_path(image_url, filter)
    // }

    // pub fn create_uniform_buffer<T>(&self, data: &T) -> CrystalResult<UniformBuffer>
    // where
    //     T: Sized,
    // {
    //     UniformBuffer::from_value(data)
    // }

    // pub fn update_uniform_buffer<T>(
    //     &self,
    //     uniform_buffer: &mut UniformBuffer,
    //     data: &T,
    // ) -> CrystalResult<()>
    // where
    //     T: Sized,
    // {
    //     unsafe {
    //         gl::BindBuffer(gl::UNIFORM_BUFFER, uniform_buffer.buffer);
    //         gl::BufferData(
    //             gl::UNIFORM_BUFFER,
    //             size_of::<T>() as isize,
    //             data as *const T as *const _,
    //             gl::DYNAMIC_DRAW,
    //         );

    //         Ok(())
    //     }
    // }

    pub fn create_vertex_buffer_with_capacity<T>(&self, count: usize) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        VertexBuffer::with_capacity::<T>(&self.device, count)
    }

    pub fn create_vertex_buffer_with_data<T>(&self, data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        VertexBuffer::with_data(&self.device, data)
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

    // pub fn create_index_buffer<T>(&self, data: &[u16]) -> CrystalResult<IndexBuffer> {
    //     IndexBuffer::from_slice(data)
    // }

    pub fn create_renderable(&self, vertex_buffers: &[(u32, &VertexBuffer)]) -> Result<Renderable> {
        Renderable::new(vertex_buffers)
    }

    // pub fn create_renderable_with_index(
    //     &self,
    //     bindings: &[Binding],
    //     index_buffer: &IndexBuffer,
    // ) -> CrystalResult<Renderable> {
    //     Renderable::from_bindings_and_index(bindings, index_buffer)
    // }

    // pub fn get_uniform_location(
    //     &self,
    //     shader: &Shader,
    //     uniform_name: &str,
    // ) -> CrystalResult<Location> {
    //     unsafe {
    //         let uniform_name_cstr = match CString::new(uniform_name) {
    //             Ok(uniform_name) => uniform_name,
    //             Err(..) => {
    //                 return Err("converting shader name to c string".into());
    //             }
    //         };

    //         let location = gl::GetUniformBlockIndex(shader.program, uniform_name_cstr.as_ptr());
    //         if location != gl::INVALID_INDEX {
    //             return Ok(location as i32);
    //         }

    //         Err(format!(
    //             "shader uniform location \"{}\" not found",
    //             uniform_name
    //         ))
    //     }
    // }

    // pub fn get_texture_location(
    //     &self,
    //     shader: &Shader,
    //     uniform_name: &str,
    // ) -> CrystalResult<Location> {
    //     unsafe {
    //         let uniform_name_cstr = match CString::new(uniform_name) {
    //             Ok(uniform_name) => uniform_name,
    //             Err(..) => {
    //                 return Err("converting shader name to c string".into());
    //             }
    //         };

    //         let location = gl::GetUniformLocation(shader.program, uniform_name_cstr.as_ptr());
    //         if location >= 0 {
    //             return Ok(location);
    //         }

    //         Err(format!(
    //             "shader texture location \"{}\" not found",
    //             uniform_name
    //         ))
    //     }
    // }

    // pub fn clear(&self) {
    //     unsafe {
    //         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    //     }
    // }

    // pub fn clear_with_color(&self, r: f32, g: f32, b: f32, a: f32) {
    //     unsafe {
    //         gl::ClearColor(r, g, b, a);
    //         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    //     }
    // }

    // pub fn use_shader(&self, shader: &Shader) {
    //     unsafe {
    //         gl::UseProgram(shader.program);
    //     }
    // }

    // pub fn use_shader_with_state(&self, shader: &Shader, state: &State) {
    //     unsafe {
    //         gl::UseProgram(shader.program);

    //         if state.depth_test {
    //             gl::Enable(gl::DEPTH_TEST);
    //         } else {
    //             gl::Disable(gl::DEPTH_TEST);
    //         }

    //         if state.depth_write {
    //             gl::DepthMask(gl::TRUE);
    //         } else {
    //             gl::DepthMask(gl::FALSE);
    //         }

    //         if state.alpha_blend {
    //             gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    //             gl::Enable(gl::BLEND);
    //         } else {
    //             gl::Disable(gl::BLEND);
    //         }

    //         let mut binding = 0;
    //         for uniform in state.uniforms.iter() {
    //             match uniform {
    //                 UniformInternal::Buffer(location, uniform_buffer_id) => {
    //                     gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, *uniform_buffer_id);
    //                     gl::UniformBlockBinding(shader.program, *location as u32, binding);
    //                 }
    //                 _ => {}
    //             }

    //             binding += 1;
    //         }
    //     }
    // }

    // pub fn draw(&self, renderable: &Renderable, vertex_count: usize, instance_count: usize) {
    //     unsafe {
    //         gl::BindVertexArray(renderable.vertex_array);
    //         gl::DrawArraysInstanced(
    //             gl::TRIANGLE_STRIP,
    //             0,
    //             vertex_count as i32,
    //             instance_count as i32,
    //         );
    //     }
    // }

    // pub fn draw_indexed(&self, renderable: &Renderable, index_count: usize, instance_count: usize) {
    //     unsafe {
    //         gl::BindVertexArray(renderable.vertex_array);
    //         gl::DrawElementsInstanced(
    //             gl::TRIANGLE_STRIP,
    //             index_count as i32,
    //             gl::UNSIGNED_SHORT,
    //             null(),
    //             instance_count as i32,
    //         );
    //     }
    // }
}
