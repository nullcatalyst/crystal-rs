use crate::shared::image::is_power_of_2;
use crate::webgl::err::Result;
use crate::TextureFilter;
use js_sys::Function;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

#[cfg(feature = "use-promise")]
use crate::webgl::internal::Promise;

#[wasm_bindgen]
pub struct Texture {
    pub(crate) context: Rc<WebGl2RenderingContext>,
    pub(crate) texture: WebGlTexture,

    #[cfg(feature = "use-promise")]
    promise: Rc<Mutex<Promise<bool>>>,
}

impl Texture {
    pub(crate) fn new(
        gl: &Rc<WebGl2RenderingContext>,
        image_path: &str,
        filter: TextureFilter,
    ) -> Result<Texture> {
        if let Some(texture) = gl.create_texture() {
            let promise = Rc::from(Mutex::from(Promise::new()));

            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

            // Because images have to be download over the internet they might
            // take a moment until they are ready. Until then put a single pixel
            // in the texture so we can use it immediately. When the image has
            // finished downloading we'll update the texture with the contents
            // of the image.
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
                Some(&[255, 255, 255, 255]),
            )?;

            let image = Rc::from(HtmlImageElement::new()?);
            image.set_onload(Some({
                let gl = Rc::clone(gl);
                let texture = texture.clone();
                let image = Rc::clone(&image);
                let promise = Rc::clone(&promise);

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

                        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

                        match filter {
                            TextureFilter::Nearest => {
                                gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
                                gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
                            }
                            TextureFilter::Linear => {
                                gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
                                gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
                            }
                            TextureFilter::MipMap => {
                                if is_power_of_2(w) && is_power_of_2(h) {
                                    gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
                                    gl.tex_parameteri(
                                        WebGl2RenderingContext::TEXTURE_2D,
                                        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                                        WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
                                    );
                                    gl.tex_parameteri(
                                        WebGl2RenderingContext::TEXTURE_2D,
                                        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
                                        WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
                                    );
                                } else {
                                    gl.tex_parameteri(
                                        WebGl2RenderingContext::TEXTURE_2D,
                                        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                                        WebGl2RenderingContext::LINEAR as i32,
                                    );
                                    gl.tex_parameteri(
                                        WebGl2RenderingContext::TEXTURE_2D,
                                        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
                                        WebGl2RenderingContext::LINEAR as i32,
                                    );
                                }
                            }
                        }

                        #[cfg(feature = "use-promise")]
                        {
                            if let Ok(mut promise) = promise.lock() {
                                promise.resolve(true);
                            }
                        }
                }))
                .dyn_into::<Function>()
                .unwrap()
            }
            ));
            image.set_src(image_path);

            Ok(Texture {
                context: Rc::clone(gl),
                texture,
                #[cfg(feature = "use-promise")]
                promise,
            })
        } else {
            Err("creating texture".into())
        }
    }
}

impl<'a> Drop for Texture {
    fn drop(&mut self) {
        self.context.delete_texture(Some(&self.texture));
    }
}

#[cfg(feature = "use-promise")]
// #[wasm_bindgen]
impl Texture {
    pub async fn promise(&mut self) {
        if let Ok(_promise) = self.promise.lock() {
            // promise.await;
        }
    }
}
