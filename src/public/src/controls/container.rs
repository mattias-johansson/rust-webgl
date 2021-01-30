extern crate wasm_bindgen;
extern crate serde;

use crate::controls::color::*;
use crate::controls::button::*;
use crate::controls::toggle_button::*;
use crate::controls::scroll_view::*;
use crate::controls::image_view::*;
use crate::controls::label::*;
use crate::controls::slider::*;
use crate::controls::control::Control;
use wasm_bindgen::prelude::*;
use serde::*;
use crate::globals::messaging::*;

use uuid::Uuid;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct Container {
    this: Uuid,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
    color: Color,
    clip: bool,
}

#[wasm_bindgen]
pub struct ContainerBuilder {
    x: Option<f32>,
    y: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    translate_x: Option<f32>,
    translate_y: Option<f32>,
    opacity: Option<f32>,
    color: Option<Color>,
    clip: bool,
}

#[wasm_bindgen]
impl Container {
/*
    pub fn add<T>(&self, object: &T) -> Result<(), JsValue> where T: Control{
        let json = serde_json::to_string(&object).unwrap();
        let message = MessageType::ObjectCreated(self.this, object.get_type(), json);
        send_message(message);
        Ok(())
    }
*/
    pub fn add_button(&self, button: &Button) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::ObjectCreated(self.this, "button".to_owned(), json);
        send_message(message);
        Ok(())
    }

    pub fn add_toggle_button(&self, button: &ToggleButton) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::ObjectCreated(self.this, "togglebutton".to_owned(), json);
        send_message(message);
        Ok(())
    }

    pub fn add_slider(&self, button: &Slider) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::ObjectCreated(self.this, "slider".to_owned(), json);
        send_message(message);
        Ok(())
    }

    pub fn add_image_view(&self, button: &ImageView) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::ObjectCreated(self.this, "imageview".to_owned(), json);
        send_message(message);
        Ok(())
    }

    pub fn add_label(&self, button: &Label) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::ObjectCreated(self.this, "label".to_owned(), json);
        send_message(message);
        Ok(())
    }

    pub fn add_scroll_view(&self, button: &ScrollView) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::ObjectCreated(self.this, "scrollview".to_owned(), json);
        send_message(message);
        Ok(())
    }

    pub fn add_container(&self, button: &Container) -> Result<(), JsValue> {
        let json = serde_json::to_string(&button).unwrap();
        let message = MessageType::ObjectCreated(self.this, "container".to_owned(), json);
        send_message(message);
        Ok(())
    }

    pub fn get_uuid(&self) -> String {
        self.this.to_string()
    }
}

impl Control for Container {
    fn get_type() -> String {
        "container".to_owned()
    }
}

#[wasm_bindgen]
impl ContainerBuilder {

   #[wasm_bindgen(constructor)]
   pub fn builder() -> ContainerBuilder { 
       let x = None;
       let y = None;
       let translate_x = None;
       let translate_y = None;
       let opacity = None;
       let width = None;
       let height = None;
       let color = None;
       let clip = false;
       ContainerBuilder { x, y, translate_x, translate_y, opacity, width, height, color, clip}
   }

   pub fn build(&self) -> Container {
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
           None => 0.0
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
       Container{ this, x, y, translate_x, translate_y, opacity, width, height, color, clip }
   }

   pub fn x(mut self, x: f32) -> ContainerBuilder {
       self.x = Some(x);
       self
   } 

   pub fn y(mut self, y: f32) -> ContainerBuilder {
       self.y = Some(y);
       self
   } 

   pub fn translate_x(mut self, translate_x: f32) -> ContainerBuilder {    
       self.translate_x = Some(translate_x);
       self
   } 

   pub fn  translate_y(mut self, translate_y: f32) -> ContainerBuilder {
       self.translate_y = Some(translate_y);
       self
   } 

   pub fn opacity(mut self, opacity: f32) -> ContainerBuilder {
       self.opacity = Some(opacity);
       self
   } 

   pub fn width(mut self, width: f32) -> ContainerBuilder {
       self.width = Some(width);
       self
   } 

   pub fn height(mut self, height: f32) -> ContainerBuilder {
       self.height = Some(height);
       self
   }

   pub fn color(mut self, color: Color) -> ContainerBuilder {
       self.color = Some(color);
       self
   }

   pub fn clip(mut self, clip: bool) -> ContainerBuilder {
       self.clip = clip;
       self
   }

}
