use crate::send_message;
use crate::MessageType;
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

#[wasm_bindgen]
impl Label {

    pub fn new(x:f32, y:f32, width:f32, height:f32, translate_x:f32, translate_y:f32, opacity:f32, text: String ) -> Label {
        let this = Uuid::new_v4();
        Label { this, x, y, width, height, translate_x, translate_y, opacity, text }
     }

    pub fn text(&mut self, text: String) {
        self.text = text;
        let json = serde_json::to_string(&self).unwrap();
        let message = MessageType::ValueUpdated(self.this, "label".to_owned(), json);
        send_message(message);
    }
  
}

