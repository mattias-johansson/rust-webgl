
extern crate wasm_bindgen;
extern crate serde;

use crate::controls::color::*;
use crate::controls::container::*;
use crate::globals::messaging::*;

use wasm_bindgen::prelude::*;
use js_sys::{Function};

use uuid::Uuid;
use serde::*;

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

    pub fn set_content(&self, button: &Container) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::SetupScrollView(self.this, json);
        send_message(message);
        Ok(())
    }
}

#[wasm_bindgen]
pub struct ScrollViewBuilder {
    x: Option<f32>,
    y: Option<f32>,
    translate_x: Option<f32>,
    translate_y: Option<f32>,
    opacity: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    color: Option<Color>,
    clip: bool,
}

#[wasm_bindgen]
impl ScrollViewBuilder {

   #[wasm_bindgen(constructor)]
   pub fn builder() -> ScrollViewBuilder { 
       let x = None;
       let y = None;
       let translate_x = None;
       let translate_y = None;
       let opacity = None;
       let width = None;
       let height = None;
       let color = None;
       let clip = false;
       ScrollViewBuilder { x, y, translate_x, translate_y, opacity, width, height, color, clip }
   }

   pub fn build(&self) -> ScrollView {
       let this = Uuid::new_v4();
       let x:f32 = match self.x {
           Some(x) => x,
           None => 0.0
       };
       let y:f32 = match self.y {
           Some(y) => y,
           None => 0.0
       };
       let translate_x:f32 = match self.translate_x {
           Some(translate_x) => translate_x,
           None => 0.0
       };
       let translate_y:f32 = match self.translate_y {
           Some(translate_y) => translate_y,
           None => 0.0
       };
       let opacity:f32 = match self.opacity {
           Some(opacity) => opacity,
           None => 1.0
       };
       let width:f32 = match self.width {
           Some(width) => width,
           None => 0.0
       };
       let height:f32 = match self.height {
           Some(height) => height,
           None => 0.0
       };
       let color: Color = match &self.color {
           Some(color) => Color {r: color.r, g: color.g, b: color.b},
           None => (Color {r: 0.0, g: 0.0, b: 0.0})
       };
       let clip = self.clip;
       ScrollView { this, x, y, translate_x, translate_y, opacity, width, height, color, clip }
   }

   pub fn x(mut self, x: f32) -> ScrollViewBuilder {
       self.x = Some(x);
       self
   } 

   pub fn y(mut self, y: f32) -> ScrollViewBuilder {
       self.y = Some(y);
       self
   } 

   pub fn translate_x(mut self, translate_x: f32) -> ScrollViewBuilder {    
       self.translate_x = Some(translate_x);
       self
   } 

   pub fn  translate_y(mut self, translate_y: f32) -> ScrollViewBuilder {
       self.translate_y = Some(translate_y);
       self
   } 

   pub fn opacity(mut self, opacity: f32) -> ScrollViewBuilder {
       self.opacity = Some(opacity);
       self
   } 

   pub fn width(mut self, width: f32) -> ScrollViewBuilder {
       self.width = Some(width);
       self
   } 

   pub fn height(mut self, height: f32) -> ScrollViewBuilder {
       self.height = Some(height);
       self
   }

   pub fn color(mut self, color: Color) -> ScrollViewBuilder {
       self.color = Some(color);
       self
   }

   pub fn clip(mut self, clip: bool) -> ScrollViewBuilder {
       self.clip = clip;
       self
   }

}
