use crate::opengl::err::Result;
use crate::opengl::*;
use crate::Binding;

pub struct CommandBuffer {
    pub(crate) clear_color: Option<(f32, f32, f32, f32)>,

    pub(crate) pipeline_index: u32,
    pub(crate) bindings: Vec<Binding>,
}

impl CommandBuffer {
    pub(crate) fn new() -> Result<CommandBuffer> {
        Ok(CommandBuffer {
            pipeline_index: 0,
            clear_color: None,
            bindings: Vec::new(),
        })
    }

    pub fn set_clear_color(&mut self, clear_color: Option<(f32, f32, f32, f32)>) {
        self.clear_color = clear_color;
    }

    pub fn use_pipeline(&mut self, pipeline: &Pipeline) {
        if let Some((red, green, blue, alpha)) = self.clear_color {
            unsafe {
                gl::ClearColor(red, green, blue, alpha);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }

        unsafe {
            if pipeline.depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            gl::DepthMask(if pipeline.depth_write {
                gl::TRUE
            } else {
                gl::FALSE
            });

            if pipeline.alpha_blend {
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Enable(gl::BLEND);
            } else {
                gl::Disable(gl::BLEND);
            }

            gl::UseProgram(pipeline.shader_program.0);
        }

        self.pipeline_index = pipeline.index;
        self.bindings = pipeline.bindings.clone();
    }

    pub fn draw(&self, renderable: &Renderable, vertex_count: usize, instance_count: usize) {
        let mut vertex_arrays = renderable.vertex_arrays.take();
        match vertex_arrays
            .iter()
            .find(|(pipeline_index, _vertex_array)| self.pipeline_index == *pipeline_index)
        {
            None => unsafe {
                let mut vertex_array = 0;
                gl::GenVertexArrays(1, &mut vertex_array);
                gl::BindVertexArray(vertex_array);

                for binding in &self.bindings {
                    if let Some((_buffer_index, buffer)) = renderable
                        .vertex_buffers
                        .iter()
                        .find(|(buffer_index, _buffer)| binding.buffer == *buffer_index)
                    {
                        gl::BindBuffer(gl::ARRAY_BUFFER, buffer.0);
                    } else {
                        continue;
                    }

                    gl::VertexAttribPointer(
                        binding.attribute,
                        4,
                        gl::FLOAT,
                        gl::FALSE,
                        binding.stride as i32,
                        binding.offset as usize as *const _,
                    );
                    gl::VertexAttribDivisor(
                        binding.attribute,
                        if binding.instanced { 1 } else { 0 },
                    );
                    gl::EnableVertexAttribArray(binding.attribute);
                }

                vertex_arrays.push((self.pipeline_index, VertexArray(vertex_array)));
            },
            Some((_pipeline_index, vertex_array)) => unsafe { gl::BindVertexArray(vertex_array.0) },
        }

        renderable.vertex_arrays.set(vertex_arrays);

        unsafe {
            gl::DrawArraysInstanced(
                gl::TRIANGLE_STRIP,
                0,
                vertex_count as i32,
                instance_count as i32,
            );
        }
    }

    // pub fn draw_indexed(&self, renderable: &Renderable, index_count: usize, instance_count: usize) {

    // }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::Finish();
        }
    }
}
