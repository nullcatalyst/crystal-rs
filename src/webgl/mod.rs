mod context;
mod index_buffer;
mod renderable;
mod shader;
mod state;
mod texture;
mod uniform_buffer;
mod vertex_buffer;

pub use crate::webgl::context::*;
pub use crate::webgl::index_buffer::*;
pub use crate::webgl::renderable::*;
pub use crate::webgl::shader::*;
pub use crate::webgl::state::*;
pub use crate::webgl::texture::*;
pub use crate::webgl::uniform_buffer::*;
pub use crate::webgl::vertex_buffer::*;

#[macro_export]
macro_rules! glsl_version {
    () => {
        "#version 300 es"
    };
}
