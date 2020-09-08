use image;
use image::RgbaImage;
use std::path::Path;

pub(crate) type BgraImage = image::ImageBuffer<image::Bgra<u8>, Vec<u8>>;

#[allow(dead_code)]
pub(crate) fn load_rgba_image<P>(file_name: P) -> Result<RgbaImage, String>
where
    P: AsRef<Path>,
{
    let img = image::open(file_name).map_err(|e| format!("loading image: {}", e))?;

    Ok(img.to_rgba())
}

#[allow(dead_code)]
pub(crate) fn load_bgra_image<P>(file_name: P) -> Result<BgraImage, String>
where
    P: AsRef<Path>,
{
    let img = image::open(file_name).map_err(|e| format!("loading image: {}", e))?;

    Ok(img.to_bgra())
}

#[allow(dead_code)]
pub(crate) fn is_power_of_2(value: u32) -> bool {
    value == (1 << (31 - value.leading_zeros()))
}
