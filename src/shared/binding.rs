#[derive(Clone, Copy)]
pub struct Binding {
    pub attribute: u32,
    pub buffer: u32,
    pub offset: usize,
    pub stride: usize,
    pub instanced: bool,
}
