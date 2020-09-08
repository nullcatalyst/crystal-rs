use crate::metal::err::Result;
use crate::shared::image::load_bgra_image;
use foreign_types::ForeignType;
use metal;
use std::ptr::null_mut;

pub struct Texture {
    pub(crate) texture: metal::Texture,
}

impl Texture {
    pub(crate) fn new(device: &metal::Device, image_path: &str) -> Result<Texture> {
        let img = load_bgra_image(image_path)?;

        let width = img.width();
        let height = img.height();

        let desc = metal::TextureDescriptor::new();
        desc.set_width(width as u64);
        desc.set_height(height as u64);
        desc.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);

        let texture = device.new_texture(&desc);
        texture.replace_region(
            metal::MTLRegion {
                origin: metal::MTLOrigin { x: 0, y: 0, z: 0 },
                size: metal::MTLSize {
                    width: width as u64,
                    height: height as u64,
                    depth: 1,
                },
            },
            0,
            img.as_ptr() as *const _,
            width as u64 * 4,
        );

        Ok(Texture { texture })
    }
}

impl Default for Texture {
    fn default() -> Texture {
        Texture {
            texture: unsafe { metal::Texture::from_ptr(null_mut()) },
        }
    }
}
