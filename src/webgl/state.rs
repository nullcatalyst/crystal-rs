use crate::webgl::texture::Texture;
use crate::webgl::uniform_buffer::UniformBuffer;
use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlTexture, WebGlUniformLocation};

#[derive(Clone)]
pub enum Location {
    Buffer(u32),
    Texture(WebGlUniformLocation),
}

pub enum Uniform<'a> {
    Buffer(Location, &'a UniformBuffer),
    Texture(Location, &'a Texture),
}

#[derive(Default, Clone, Copy)]
pub struct StateDesc<'a> {
    pub depth_test: bool,
    pub depth_write: bool,
    pub alpha_blend: bool,
    pub uniforms: &'a [Uniform<'a>],
}

pub struct State {
    pub(crate) depth_test: bool,
    pub(crate) depth_write: bool,
    pub(crate) alpha_blend: bool,
    pub(crate) uniforms: Vec<UniformInternal>,
}

impl State {
    pub fn from_desc(desc: &StateDesc) -> Result<State, JsValue> {
        let mut uniforms = Vec::with_capacity(desc.uniforms.len());
        for uniform in desc.uniforms.iter() {
            uniforms.push(match uniform {
                Uniform::Buffer(location, uniform_buffer) => {
                    UniformInternal::Buffer(location.clone(), uniform_buffer.buffer.clone())
                }
                Uniform::Texture(location, texture) => {
                    UniformInternal::Texture(location.clone(), texture.texture.clone())
                }
            });
        }

        Ok(State {
            depth_test: desc.depth_test,
            depth_write: desc.depth_write,
            alpha_blend: desc.alpha_blend,
            uniforms,
        })
    }
}

pub(crate) enum UniformInternal {
    Buffer(Location, WebGlBuffer),
    Texture(Location, WebGlTexture),
}
