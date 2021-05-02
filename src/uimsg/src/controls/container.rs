use wasm_bindgen::prelude::*;
use serde::*;
use uuid::Uuid;

use crate::MessageType;
use crate::Button;
use crate::ToggleButton;
use crate::Label;
use crate::ImageView;
use crate::ScrollView;
use crate::Slider;
use crate::Color;

use crate::send_message;

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
impl Container {

    pub fn new(x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color: Color, clip:bool) -> Container {

       let this = Uuid::new_v4();
        Container {this, x, y, translate_x, translate_y, opacity, width, height, color, clip }
    }
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

    pub fn remove_container(&self, button: &Container) -> Result<(), JsValue> {
        let message = MessageType::ObjectRemoved(self.this, button.this);
        send_message(message);
        Ok(())
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

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        let message = MessageType::ValueUpdated(self.this, "width".to_owned(), width.to_string());
        send_message(message);
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
        let message = MessageType::ValueUpdated(self.this, "height".to_owned(), height.to_string());
        send_message(message);
    }

    pub fn set_translate_x(&mut self, translate_x: f32) {
        self.translate_x = translate_x;
        let message = MessageType::ValueUpdated(self.this, "translate_x".to_owned(), translate_x.to_string());
        send_message(message);
    }

    pub fn set_translate_y(&mut self, translate_y: f32) {
        self.translate_y = translate_y;
        let message = MessageType::ValueUpdated(self.this, "translate_y".to_owned(), translate_y.to_string());
        send_message(message);
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        let message = MessageType::ValueUpdated(self.this, "opacity".to_owned(), opacity.to_string());
        send_message(message);
    }
 
}

