use wasm_bindgen::prelude::*;

use crate::send_message;
use crate::MessageType;

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
    pub fn get_uuid(&self) -> String {
        self.this.to_string()
    }


    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        let message = MessageType::ValueUpdated(self.this, "x".to_owned(), x.to_string());
        send_message(message);
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        let message = MessageType::ValueUpdated(self.this, "y".to_owned(), y.to_string());
        send_message(message);
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        let message = MessageType::ValueUpdated(self.this, "opacity".to_owned(), opacity.to_string());
        send_message(message);
    }
 
}
