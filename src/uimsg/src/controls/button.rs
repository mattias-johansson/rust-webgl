
use wasm_bindgen::prelude::*;
use serde::*;
use uuid::Uuid;

use crate::send_message;
use crate::MessageType;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Button {
    this: Uuid,
    text: String, 
    x: f32, 
    y: f32, 
    opacity: f32,
    state: ButtonState,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ButtonState {
    NotPressed,
    Pressed,
    ToPressed,
    ToNotPressed,
}

impl Button {

    pub fn new(this:Uuid, x:f32, y:f32, opacity:f32, text:String, state:ButtonState) -> Button {
        Button { this, x, y, opacity, text, state }
    }
    pub fn get_uuid(&self) -> String {
        self.this.to_string()
    }
    pub fn this(&self) -> Uuid {
        self.this
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn opacity(&self) -> f32 {
        self.opacity
    }
    pub fn text(&self) -> String {
        self.text.as_str().to_owned()
    }
    pub fn state(&self) -> ButtonState {
        self.state
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