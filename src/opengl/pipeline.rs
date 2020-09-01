use crate::opengl::err::Result;
use crate::opengl::{Program, Shader};
use crate::shared::{Binding, PipelineDesc};
use std::rc::Rc;

pub struct Pipeline {
    /// An internal counter used by renderables to match the pipeline with a corresponding vertex
    /// array object.
    pub(crate) index: u32,
    pub(crate) shader_program: Rc<Program>,
    pub(crate) depth_test: bool,
    pub(crate) depth_write: bool,
    pub(crate) alpha_blend: bool,
    pub(crate) bindings: Vec<Binding>,
}

impl Pipeline {
    pub(crate) fn new(shader: &Shader, desc: &PipelineDesc) -> Result<Pipeline> {
        static mut NEXT_INDEX: u32 = 0;

        let index = unsafe {
            let index = NEXT_INDEX;
            NEXT_INDEX += 1;

            index
        };

        Ok(Pipeline {
            index,
            shader_program: shader.program.clone(),
            depth_test: desc.depth_test,
            depth_write: desc.depth_write,
            alpha_blend: desc.alpha_blend,
            bindings: Vec::from(desc.bindings),
        })
    }
}
