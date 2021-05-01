use crate::send_message;
use crate::MessageType;
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

#[wasm_bindgen]
impl ImageView {

    pub fn new(x:f32, y:f32, width:f32, height:f32, translate_x:f32, translate_y:f32, opacity:f32, image: String ) -> ImageView {
        let this = Uuid::new_v4();
        ImageView { this, x, y, width, height, translate_x, translate_y, opacity, image }
     }

    pub fn opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        let json = serde_json::to_string(&self).unwrap();
        let message = MessageType::ValueUpdated(self.this, "opacity".to_owned(), json);
        send_message(message);

    }
  
}
