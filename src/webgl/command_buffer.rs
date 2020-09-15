use crate::webgl::err::Result;
use crate::webgl::internal::*;
use crate::webgl::*;
use crate::Binding;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

pub struct CommandBuffer {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) clear_color: Option<(f32, f32, f32, f32)>,
    pub(crate) pipeline_index: u32,
    /// A reference to the shader program is needed to be able to set the
    /// uniform block binding.
    pub(crate) shader_program: Option<Rc<Program>>,
    pub(crate) bindings: Vec<Binding>,
}

impl CommandBuffer {
    pub(crate) fn new(gl: &Rc<WebGl2RenderingContext>) -> Result<CommandBuffer> {
        Ok(CommandBuffer {
            context: Rc::clone(gl),
            clear_color: None,
            pipeline_index: 0,
            shader_program: None,
            bindings: Vec::new(),
        })
    }

    pub fn set_clear_color(&mut self, clear_color: Option<(f32, f32, f32, f32)>) {
        self.clear_color = clear_color;
    }

    pub fn use_pipeline(&mut self, pipeline: &Pipeline) {
        let gl = &self.context;

        if let Some((red, green, blue, alpha)) = self.clear_color {
            gl.clear_color(red, green, blue, alpha);
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        }

        if pipeline.depth_test {
            gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        } else {
            gl.disable(WebGl2RenderingContext::DEPTH_TEST);
        }

        gl.depth_mask(pipeline.depth_write);

        if pipeline.alpha_blend {
            gl.blend_func(
                WebGl2RenderingContext::SRC_ALPHA,
                WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
            );
            gl.enable(WebGl2RenderingContext::BLEND);
        } else {
            gl.disable(WebGl2RenderingContext::BLEND);
        }

        gl.use_program(Some(&pipeline.shader_program.program));

        self.pipeline_index = pipeline.index;
        self.shader_program = Some(Rc::clone(&pipeline.shader_program));
        self.bindings = pipeline.bindings.clone();
    }

    pub fn use_uniform(&mut self, uniform_buffer: &UniformBuffer, location: u32, binding: u32) {
        if let Some(shader_program) = &self.shader_program {
            let gl = &self.context;
            gl.bind_buffer_base(
                WebGl2RenderingContext::UNIFORM_BUFFER,
                binding,
                Some(&uniform_buffer.buffer.buffer),
            );
            gl.uniform_block_binding(&shader_program.program, location, binding);
        }
    }

    pub fn use_texture(&mut self, texture: &Texture, location: WebGlUniformLocation, binding: u32) {
        let gl = &self.context;
        gl.uniform1i(Some(&location), binding as i32);
        gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture.texture));
    }

    pub fn draw(&mut self, renderable: &Renderable, vertex_count: usize, instance_count: usize) {
        let gl = &self.context;
        let mut vertex_arrays = renderable.vertex_arrays.take();
        match vertex_arrays
            .iter()
            .find(|(pipeline_index, _vertex_array)| self.pipeline_index == *pipeline_index)
        {
            None => {
                if let Some(vertex_array) = gl.create_vertex_array() {
                    gl.bind_vertex_array(Some(&vertex_array));

                    for binding in &self.bindings {
                        if let Some((_buffer_index, buffer)) = renderable
                            .vertex_buffers
                            .iter()
                            .find(|(buffer_index, _buffer)| binding.buffer == *buffer_index)
                        {
                            gl.bind_buffer(
                                WebGl2RenderingContext::ARRAY_BUFFER,
                                Some(&buffer.buffer),
                            );
                        } else {
                            continue;
                        }

                        gl.vertex_attrib_pointer_with_i32(
                            binding.attribute,
                            4,
                            WebGl2RenderingContext::FLOAT,
                            false,
                            binding.stride as i32,
                            binding.offset as i32,
                        );
                        gl.vertex_attrib_divisor(
                            binding.attribute,
                            if binding.instanced { 1 } else { 0 },
                        );
                        gl.enable_vertex_attrib_array(binding.attribute);
                    }

                    vertex_arrays.push((
                        self.pipeline_index,
                        VertexArray {
                            context: Rc::clone(&gl),
                            vertex_array,
                        },
                    ));
                } else {
                    panic!("drawing renderable: creating webgl vertex array object");
                }
            }
            Some((_pipeline_index, vertex_array)) => {
                gl.bind_vertex_array(Some(&vertex_array.vertex_array))
            }
        }

        renderable.vertex_arrays.set(vertex_arrays);

        gl.draw_arrays_instanced(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0,
            vertex_count as i32,
            instance_count as i32,
        );
    }

    // pub fn draw_indexed(&self, renderable: &Renderable, index_count: usize, instance_count: usize) {

    // }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        self.context.finish();
    }
}
