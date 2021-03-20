use wasm_bindgen::prelude::*;
use serde::*;
use uuid::Uuid;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct Label {
    this: Uuid,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
    text: String,
}

impl Label {

    pub fn new(this:Uuid, x:f32, y:f32, width:f32, height:f32, translate_x:f32, translate_y:f32, opacity:f32, text: String ) -> Label {
        Label { this, x, y, width, height, translate_x, translate_y, opacity, text }
     }
  
}

