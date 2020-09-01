use crate::metal::err::Result;
use crate::metal::VertexBuffer;
use metal;
// use crate::metal::index_buffer::IndexBuffer;

pub struct Renderable {
    pub(crate) vertex_buffers: Vec<(u32, metal::Buffer)>,
}

impl Renderable {
    pub(crate) fn new(vertex_buffers: &[(u32, &VertexBuffer)]) -> Result<Renderable> {
        Ok(Renderable {
            vertex_buffers: vertex_buffers
                .iter()
                .map(|(i, vertex_buffer)| (*i, vertex_buffer.buffer.clone()))
                .collect(),
        })
    }

    // pub(crate) fn from_bindings_and_index(
    //     bindings: &[Binding<VertexBuffer>],
    //     index_buffer: &IndexBuffer,
    // ) -> CrystalResult<Renderable> {
    //     unsafe {
    //         let mut vertex_array = 0;
    //         gl::GenVertexArrays(1, &mut vertex_array);
    //         gl::BindVertexArray(vertex_array);

    //         for binding in bindings {
    //             gl::BindBuffer(gl::ARRAY_BUFFER, binding.buffer.buffer);
    //             gl::VertexAttribPointer(
    //                 binding.attribute,
    //                 4,
    //                 gl::FLOAT,
    //                 gl::FALSE,
    //                 binding.stride as i32,
    //                 binding.offset as usize as *const _,
    //             );
    //             gl::VertexAttribDivisor(binding.attribute, if binding.instanced { 1 } else { 0 });
    //             gl::EnableVertexAttribArray(binding.attribute);
    //         }

    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.buffer);

    //         Ok(Renderable { vertex_array })
    //     }
    // }
}
