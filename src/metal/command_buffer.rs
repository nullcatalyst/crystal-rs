use crate::metal::err::Result;
use crate::metal::*;

pub struct CommandBuffer {
    pub(crate) screen: metal::CoreAnimationDrawable,
    pub(crate) clear_color: Option<(f32, f32, f32, f32)>,
    pub(crate) command_buffer: metal::CommandBuffer,
    pub(crate) encoder: Option<metal::RenderCommandEncoder>,
}

impl CommandBuffer {
    pub(crate) fn new(
        screen: &metal::CoreAnimationDrawableRef,
        command_buffer: &metal::CommandBufferRef,
    ) -> Result<CommandBuffer> {
        Ok(CommandBuffer {
            screen: screen.to_owned(),
            clear_color: None,
            command_buffer: command_buffer.to_owned(),
            encoder: None,
        })
    }

    pub fn set_clear_color(&mut self, clear_color: Option<(f32, f32, f32, f32)>) {
        self.clear_color = clear_color;
    }

    pub fn use_pipeline(&mut self, pipeline: &Pipeline) {
        let render_pass_desc = metal::RenderPassDescriptor::new();
        let color_attachment = render_pass_desc.color_attachments().object_at(0).unwrap();
        color_attachment.set_texture(Some(self.screen.texture()));

        if let Some((red, green, blue, alpha)) = self.clear_color {
            color_attachment.set_load_action(metal::MTLLoadAction::Clear);
            color_attachment.set_clear_color(metal::MTLClearColor::new(
                red as f64,
                green as f64,
                blue as f64,
                alpha as f64,
            ));
        }
        color_attachment.set_store_action(metal::MTLStoreAction::Store);

        let encoder = self
            .command_buffer
            .new_render_command_encoder(render_pass_desc)
            .to_owned();

        // encoder.set_scissor_rect(metal::MTLScissorRect {
        //     x: 0,
        //     y: 0,
        //     width: 100,
        //     height: 100,
        // });

        encoder.set_render_pipeline_state(&pipeline.pipeline_state);
        self.encoder = Some(encoder);
    }

    pub fn use_uniform(&mut self, uniform_buffer: &UniformBuffer, id: u32) {
        if let Some(encoder) = &self.encoder {
            encoder.set_vertex_buffer(id as u64, Some(&uniform_buffer.buffer), 0);
            encoder.set_fragment_buffer(id as u64, Some(&uniform_buffer.buffer), 0);
        }
    }

    pub fn use_texture(&mut self, texture: &Texture, id: u32) {
        if let Some(encoder) = &self.encoder {
            encoder.set_fragment_texture(id as u64, Some(&texture.texture));
        }
    }

    pub fn draw(&mut self, renderable: &Renderable, vertex_count: usize, instance_count: usize) {
        if let Some(encoder) = &self.encoder {
            for (i, buffer) in &renderable.vertex_buffers {
                encoder.set_vertex_buffer(*i as u64, Some(&buffer), 0);
            }

            encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::TriangleStrip,
                0,
                vertex_count as u64,
                instance_count as u64,
            );
        }
    }

    // pub fn draw_indexed(&self, renderable: &Renderable, index_count: usize, instance_count: usize) {}
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        if let Some(encoder) = &self.encoder {
            encoder.end_encoding();
        }

        let command_buffer = self.command_buffer.as_ref();
        command_buffer.present_drawable(&self.screen);
        command_buffer.commit();
    }
}
