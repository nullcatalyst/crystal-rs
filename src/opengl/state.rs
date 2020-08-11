use crate::opengl::texture::Texture;
use crate::opengl::uniform_buffer::UniformBuffer;

pub type Location = i32;

pub enum Uniform<'a> {
    Buffer(Location, &'a UniformBuffer),
    Texture(Location, &'a Texture),
}

#[derive(Default, Clone, Copy)]
pub struct StateDesc<'a> {
    pub uniforms: &'a [Uniform<'a>],
    pub depth_test: bool,
    pub depth_write: bool,
    pub alpha_blend: bool,
}

pub struct State {
    pub(crate) uniforms: Vec<UniformInternal>,
    pub(crate) depth_test: bool,
    pub(crate) depth_write: bool,
    pub(crate) alpha_blend: bool,
}

impl State {
    pub fn from_desc(desc: &StateDesc) -> Result<State, String> {
        let mut uniforms = Vec::with_capacity(desc.uniforms.len());
        for uniform in desc.uniforms.iter() {
            uniforms.push(match uniform {
                Uniform::Buffer(location, uniform_buffer) => {
                    UniformInternal::Buffer(*location, uniform_buffer.buffer)
                }
                Uniform::Texture(location, texture) => {
                    UniformInternal::Texture(*location, texture.texture)
                }
            });
        }

        Ok(State {
            uniforms,
            depth_test: desc.depth_test,
            depth_write: desc.depth_write,
            alpha_blend: desc.alpha_blend,
        })
    }
}

pub(crate) enum UniformInternal {
    Buffer(Location, u32),
    Texture(Location, u32),
}
