use gl;
use image;
use image::RgbaImage;
use std::path::Path;

pub enum TextureFilter {
    Nearest,
    Linear,
    MipMap,
}

pub struct Texture {
    pub(crate) texture: u32,
}

impl<'a> Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture);
        }
    }
}

impl Texture {
    pub(crate) fn from_path<P>(file_name: P, filter: TextureFilter) -> Result<Texture, String>
    where
        P: AsRef<Path>,
    {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            let img = load_image(file_name);
            let w = img.width();
            let h = img.height();

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                w as i32,
                h as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.into_raw().as_ptr() as *const _,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            match filter {
                TextureFilter::Nearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                }
                TextureFilter::Linear => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                }
                TextureFilter::MipMap => {
                    if is_power_of_2(w) && is_power_of_2(h) {
                        gl::GenerateMipmap(gl::TEXTURE_2D);
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_MIN_FILTER,
                            gl::LINEAR_MIPMAP_LINEAR as i32,
                        );
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_MAG_FILTER,
                            gl::LINEAR_MIPMAP_LINEAR as i32,
                        );
                    } else {
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_MIN_FILTER,
                            gl::LINEAR as i32,
                        );
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_MAG_FILTER,
                            gl::LINEAR as i32,
                        );
                    }
                }
            }

            Ok(Texture { texture })
        }
    }
}

fn load_image<P>(file_name: P) -> RgbaImage
where
    P: AsRef<Path>,
{
    let img = image::open(file_name).unwrap();
    img.as_rgba8().unwrap().clone()
}

fn is_power_of_2(value: u32) -> bool {
    value == (1 << (31 - value.leading_zeros()))
}
