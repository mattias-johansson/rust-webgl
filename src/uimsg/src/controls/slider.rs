use wasm_bindgen::prelude::*;

use uuid::Uuid;
use serde::*;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Slider {
    this: Uuid,
    x: f32, 
    y: f32, 
    opacity: f32,
    text: String, 
}

impl Slider {

    pub fn new(this:Uuid, x:f32, y:f32, opacity:f32, text:String) -> Slider {
        Slider { this, x, y, opacity, text }
    }
}
