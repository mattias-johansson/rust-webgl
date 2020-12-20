extern crate wasm_bindgen;
extern crate serde;

use wasm_bindgen::prelude::*;
use serde::*;

#[derive(PartialEq, Eq, Clone, Copy, Serialize)]
enum ButtonState {
    NotPressed,
    Pressed,
    ToPressed,
    ToNotPressed,
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct Button {
    text: String, 
    x: f32, 
    y: f32, 
    opacity: f32,
    state: ButtonState,
}

#[wasm_bindgen]
pub struct ButtonBuilder {
    x: Option<f32>,
    y: Option<f32>,
    opacity: Option<f32>,
    text: Option<String>
}

#[wasm_bindgen]
impl ButtonBuilder {

    #[wasm_bindgen(constructor)]
    pub fn builder() -> ButtonBuilder { 
        let x = None;
        let y = None;
        let opacity = None;
        let text = None;
        ButtonBuilder { x, y, opacity, text }
    }

    pub fn build(&self) -> Button {
        let x:f32 = match self.x {
            Some(x) => x,
            None => 0.0
        };
        let y:f32 = match self.y {
            Some(y) => y,
            None => 0.0
        };
        let opacity:f32 = match self.opacity {
            Some(opacity) => opacity,
            None => 1.0
        };
        let text:String = match &self.text {
            Some(text) => text.to_string(),
            None => "".to_owned()
        };
        let state = ButtonState::NotPressed;
        Button {x,y,opacity,text,state}
    }

    pub fn x(mut self, x: f32) -> ButtonBuilder {
        self.x = Some(x);
        self
    } 
    
    pub fn y(mut self, y: f32) -> ButtonBuilder {
        self.y = Some(y);
        self
    } 
    
    pub fn opacity(mut self, opacity: f32) -> ButtonBuilder {
        self.opacity = Some(opacity);
        self
    } 
    
    pub fn text(mut self, text: &str) -> ButtonBuilder {
        let owned_text = text.to_owned();
        self.text = Some(owned_text);
        self
    } 
     
}