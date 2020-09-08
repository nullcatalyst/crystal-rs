use crate::metal::err::Result;
use metal;
use std::str;

pub struct Library {
    pub(crate) library: metal::Library,
}

impl Library {
    pub(crate) fn new(device: &metal::Device, library_path: &str) -> Result<Library> {
        let library = device.new_library_with_file(library_path)?;

        Ok(Library { library })
    }
}

pub struct Shader {
    pub(crate) vertex_function: metal::Function,
    pub(crate) fragment_function: metal::Function,
}

impl Shader {
    pub(crate) fn new(
        library: &metal::Library,
        vertex_name: &str,
        fragment_name: &str,
    ) -> Result<Shader> {
        let vertex_function = library.get_function(vertex_name, None)?;
        let fragment_function = library.get_function(fragment_name, None)?;

        Ok(Shader {
            vertex_function,
            fragment_function,
        })
    }
}
