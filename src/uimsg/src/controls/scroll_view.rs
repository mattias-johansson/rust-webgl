use crate::send_message;
use wasm_bindgen::prelude::*;
use serde::*;
use uuid::Uuid;

use crate::Container;
use crate::Color;
use crate::MessageType;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct ScrollView {
    this: Uuid,
    x: f32,
    y: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
    width: f32,
    height: f32,
    color: Color,
    clip: bool,
}

#[wasm_bindgen]
impl ScrollView {

    pub fn new(x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color: Color, clip:bool ) -> ScrollView {
       let this = Uuid::new_v4();
       ScrollView { this, x, y, translate_x, translate_y, opacity, width, height, color, clip }
    }
 
    pub fn set_content(&self, button: &Container) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::SetupScrollView(self.this, json);
        send_message(message);
        Ok(())
    }
}
