use wasm_bindgen::prelude::*;

use uuid::Uuid;
use serde::*;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct ToggleButton {
    this: Uuid,
    text: String, 
    x: f32, 
    y: f32, 
    opacity: f32,
    state: ToggleButtonState
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Serialize)]
pub enum ToggleButtonState {
    On,
    Off,
    ToOn,
    ToOff,
}

impl ToggleButton {
    
    pub fn new(this:Uuid, x:f32, y:f32, opacity:f32, text:String, state:ToggleButtonState) -> ToggleButton {
        ToggleButton { this, x, y, opacity, text, state }
    }
}