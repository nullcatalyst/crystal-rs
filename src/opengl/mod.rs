mod context;
mod err;
mod index_buffer;
mod renderable;
mod shader;
mod state;
mod texture;
mod uniform_buffer;
mod vertex_buffer;

pub use crate::opengl::context::*;
pub use crate::opengl::index_buffer::*;
pub use crate::opengl::renderable::*;
pub use crate::opengl::shader::*;
pub use crate::opengl::state::*;
pub use crate::opengl::texture::*;
pub use crate::opengl::uniform_buffer::*;
pub use crate::opengl::vertex_buffer::*;

#[macro_export]
macro_rules! glsl_version {
    () => {
        "#version 330"
    };
}
