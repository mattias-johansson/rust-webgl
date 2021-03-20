use wasm_bindgen::prelude::*;
use serde::*;
use uuid::Uuid;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct ImageView {
    this: Uuid,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
    image: String,
}

impl ImageView {

    pub fn new(this:Uuid, x:f32, y:f32, width:f32, height:f32, translate_x:f32, translate_y:f32, opacity:f32, image: String ) -> ImageView {
        ImageView { this, x, y, width, height, translate_x, translate_y, opacity, image }
     }
  
}
