use crate::metal::err::Result;
use crate::metal::*;
use crate::{PipelineDesc, TextureFilter};
use metal;

#[cfg(any(feature = "use-sdl2"))]
use cocoa::appkit::NSWindow;
#[cfg(any(feature = "use-sdl2", feature = "use-winit"))]
use cocoa::{appkit::NSView, base::id as cocoa_id};
#[cfg(any(feature = "use-sdl2", feature = "use-winit"))]
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
}

impl Context {
    pub fn new(device: metal::Device, layer: metal::CoreAnimationLayer) -> Result<Context> {
        let command_queue = device.new_command_queue();

        Ok(Context {
            device,
            layer,
            command_queue,
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

        // let (width, height) = window.size();
        // layer.set_drawable_size(metal::CGSize::new(width as f64, height as f64));

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

    pub fn create_library(&mut self, library_path: &str) -> Result<Library> {
        Library::new(&self.device, library_path)
    }

    pub fn create_shader(
        &mut self,
        library: &Library,
        vertex_name: &str,
        fragment_name: &str,
    ) -> Result<Shader> {
        Shader::new(&library.library, vertex_name, fragment_name)
    }

    pub fn create_pipeline(&mut self, shader: &Shader, desc: &PipelineDesc) -> Result<Pipeline> {
        Pipeline::new(&self.device, shader, desc)
    }

    pub fn create_texture(&mut self, image_path: &str) -> Result<Texture> {
        Texture::new(&self.device, image_path)
    }

    pub fn create_texture_with_filter(
        &mut self,
        image_path: &str,
        _filter: TextureFilter,
    ) -> Result<Texture> {
        Texture::new(&self.device, image_path)
    }

    pub fn create_uniform_buffer_with_capacity(
        &mut self,
        capacity: usize,
    ) -> Result<UniformBuffer> {
        UniformBuffer::with_capacity(&self.device, capacity)
    }

    pub fn create_uniform_buffer_with_value<T>(&mut self, value: &T) -> Result<UniformBuffer>
    where
        T: Sized,
    {
        UniformBuffer::with_data(&self.device, value)
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

    pub fn create_vertex_buffer_with_capacity(&mut self, capacity: usize) -> Result<VertexBuffer> {
        VertexBuffer::with_capacity(&self.device, capacity)
    }

    pub fn create_vertex_buffer_with_data<T>(&mut self, data: &[T]) -> Result<VertexBuffer>
    where
        T: Sized,
    {
        VertexBuffer::with_data(&self.device, data)
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

    // pub fn create_index_buffer<T>(&mut self, data: &[u16]) -> CrystalResult<IndexBuffer> {
    //     IndexBuffer::from_slice(data)
    // }

    pub fn create_renderable(
        &mut self,
        vertex_buffers: &[(u32, &VertexBuffer)],
    ) -> Result<Renderable> {
        Renderable::new(vertex_buffers)
    }

    // pub fn create_renderable_with_index(
    //     &mut self,
    //     bindings: &[Binding],
    //     index_buffer: &IndexBuffer,
    // ) -> CrystalResult<Renderable> {
    //     Renderable::from_bindings_and_index(bindings, index_buffer)
    // }
}
