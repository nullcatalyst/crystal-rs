use crate::metal::err::Result;
use crate::metal::shader::Shader;
use crate::shared::{Binding, PipelineDesc};

pub struct Pipeline {
    // pub(crate) clear_color: Option<(f32, f32, f32, f32)>,
    // pub(crate) clear_depth: Option<f32>,
    pub(crate) pipeline_state: metal::RenderPipelineState,
    // pub(crate) bindings: Vec<Binding>,
}

impl Pipeline {
    pub(crate) fn new(
        device: &metal::Device,
        shader: &Shader,
        desc: &PipelineDesc,
    ) -> Result<Pipeline> {
        let pipeline_state_desc = metal::RenderPipelineDescriptor::new();
        pipeline_state_desc.set_vertex_function(Some(&shader.vertex_function));
        pipeline_state_desc.set_fragment_function(Some(&shader.fragment_function));

        let attachment = pipeline_state_desc
            .color_attachments()
            .object_at(0)
            .unwrap();
        attachment.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);

        let vertex_desc = metal::VertexDescriptor::new();
        let attributes = vertex_desc.attributes();
        let layouts = vertex_desc.layouts();
        for (i, binding) in desc.bindings.iter().enumerate() {
            if let Some(attr) = attributes.object_at(i as u64) {
                attr.set_format(metal::MTLVertexFormat::Float4);
                attr.set_buffer_index(binding.buffer as u64);
                attr.set_offset(binding.offset as u64);
            }

            if let Some(layout) = layouts.object_at(binding.buffer as u64) {
                layout.set_stride(binding.stride as u64);
                layout.set_step_rate(1);
                layout.set_step_function(if binding.instanced {
                    metal::MTLVertexStepFunction::PerInstance
                } else {
                    metal::MTLVertexStepFunction::PerVertex
                });
            }
        }
        pipeline_state_desc.set_vertex_descriptor(Some(&vertex_desc));

        if desc.alpha_blend {
            attachment.set_blending_enabled(true);
            attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
            attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
            attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
            attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
            attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
            attachment
                .set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
        }

        let pipeline_state = device.new_render_pipeline_state(&pipeline_state_desc)?;

        Ok(Pipeline {
            // bindings: Vec::from(desc.bindings),
            pipeline_state,
        })
    }
}
