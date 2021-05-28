use wasm_bindgen::prelude::*;
use serde::*;
use uuid::Uuid;

use crate::send_message;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct ListView {
    this: Uuid,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
    
}

#[wasm_bindgen]
impl ListView {

    pub fn get_uuid(&self) -> String {
        self.this.to_string()
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

}