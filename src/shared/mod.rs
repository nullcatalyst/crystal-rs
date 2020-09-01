#[derive(Default, Clone, Copy)]
pub struct PipelineDesc<'a> {
    pub depth_test: bool,
    pub depth_write: bool,
    pub alpha_blend: bool,
    pub bindings: &'a [Binding],
}

#[derive(Clone, Copy)]
pub struct Binding {
    pub attribute: u32,
    pub buffer: u32,
    pub offset: usize,
    pub stride: usize,
    pub instanced: bool,
}
