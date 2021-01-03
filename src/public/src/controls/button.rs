extern crate wasm_bindgen;
extern crate serde;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use js_sys::{Function};

use uuid::Uuid;
use serde::*;

use crate::globals::messaging::*;

#[derive(PartialEq, Eq, Clone, Copy, Serialize)]
enum ButtonState {
    NotPressed,
    Pressed,
    ToPressed,
    ToNotPressed,
}

pub struct ButtonSignal {
    this: Uuid,
    onClicked: Option<Function>
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct Button {
    this: Uuid,
    text: String, 
    x: f32, 
    y: f32, 
    opacity: f32,
    state: ButtonState
}

#[wasm_bindgen]
pub struct ButtonBuilder {
    x: Option<f32>,
    y: Option<f32>,
    opacity: Option<f32>,
    text: Option<String>,
    callback: Option<Function>
}

#[wasm_bindgen]
impl ButtonBuilder {

    #[wasm_bindgen(constructor)]
    pub fn builder() -> ButtonBuilder { 
        let x = None;
        let y = None;
        let opacity = None;
        let text = None;
        let callback = None;
        ButtonBuilder { x, y, opacity, text, callback }
    }

    pub fn build(&mut self) -> Button {
        let this = Uuid::new_v4();
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
        let callback = self.callback.take();
        if callback.is_some() {
            let mut signal: HashMap<String, Function> = HashMap::new();
            signal.insert("onClicked".to_owned(), callback.unwrap());
            let mut listener = HashMap::new();
            listener.insert(this, signal);

            web_sys::console::log_1(&this.to_string().into()); 
            add_on_message_handler(listener);
        }
        Button { this, x, y, opacity, text, state }
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
     
    pub fn callback(mut self, callback: Function) -> ButtonBuilder {
        self.callback = Some(callback);
        self
    } 
}