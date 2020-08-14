use crate::webgl::err::CrystalResult;
use js_sys::Function;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

pub struct Texture {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) texture: WebGlTexture,
}

impl<'a> Drop for Texture {
    fn drop(&mut self) {
        self.context.delete_texture(Some(&self.texture));
    }
}

impl Texture {
    pub(crate) fn from_url(
        gl: &Rc<WebGl2RenderingContext>,
        image_url: &str,
    ) -> CrystalResult<Texture> {
        if let Some(texture) = gl.create_texture() {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

            // Because images have to be download over the internet they might take a moment until
            // they are ready. Until then put a single pixel in the texture so we can use it
            // immediately. When the image has finished downloading we'll update the texture with
            // the contents of the image.
            // const pixel = new Uint8Array([0, 0, 255, 255]);  // opaque blue
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                1,
                1,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(&[0, 0, 255, 255]),
            )?;

            let image = Rc::from(HtmlImageElement::new()?);
            image.set_onload(Some({
                let gl = Rc::clone(gl);
                let texture = texture.clone();
                let image = Rc::clone(&image);

                &Closure::once_into_js(Box::from(move || {
                        let w = image.width();
                        let h = image.height();

                        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
                        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_image_element(
                            WebGl2RenderingContext::TEXTURE_2D,
                            0,
                            WebGl2RenderingContext::RGBA as i32,
                            w as i32,
                            h as i32,
                            0,
                            WebGl2RenderingContext::RGBA,
                            WebGl2RenderingContext::UNSIGNED_BYTE,
                            &image,
                        ).unwrap();

                        // WebGL1 has different requirements for power of 2 images vs non power of 2
                        // images so check if the image is a power of 2 in both dimensions.
                        if is_power_of_2(w) && is_power_of_2(h) {
                            // Yes, it's a power of 2. Generate mips.
                            gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

                            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32);
                        } else {
                            // No, it's not a power of 2. Turn off mip maps and set wrapping to
                            // clamp to edge.
                            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
                        }
                }))
                .dyn_into::<Function>()
                .unwrap()
            }
            ));
            image.set_src(image_url);

            Ok(Texture {
                context: Rc::clone(gl),
                texture,
            })
        } else {
            Err("failed to create texture".into())
        }
    }
}

fn is_power_of_2(value: u32) -> bool {
    value == (1 << (31 - value.leading_zeros()))
}
