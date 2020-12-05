extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
} 


#[wasm_bindgen]
impl Color {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Color {
//        panic!("Error");
        web_sys::console::log_1(&"Creating color".into());
        Color { r: 0.5, g: 0.4, b: 0.3}
    }
}

