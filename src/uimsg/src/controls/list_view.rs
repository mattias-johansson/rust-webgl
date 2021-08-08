use wasm_bindgen::prelude::*;
use serde::*;
use uuid::Uuid;

use crate::send_message;
use crate::MessageType;

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

    pub fn new(x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32) -> ListView {
        let this = Uuid::new_v4();
        ListView { this, x, y, width, height, translate_x, translate_y, opacity}
     }
    
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

    pub fn set_content(&self, json: &str) -> Result<(), JsValue> {
        let json = json.to_owned(); // add JSON validation
        let message = MessageType::SetupListView(self.this, json);
        send_message(message);
        Ok(())
    }

}