use wasm_bindgen::JsValue;

pub type CrystalResult<T> = std::result::Result<T, JsValue>;
