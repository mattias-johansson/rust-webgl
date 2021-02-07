use wasm_bindgen::prelude::*;
use serde::*;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Serialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
} 

#[wasm_bindgen]
impl Color {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }
}

