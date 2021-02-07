use wasm_bindgen::prelude::*;

use uuid::Uuid;
use serde::*;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Slider {
    this: Uuid,
    text: String, 
    x: f32, 
    y: f32, 
    opacity: f32,
}


#[wasm_bindgen]
pub struct SliderBuilder {
    x: Option<f32>,
    y: Option<f32>,
    opacity: Option<f32>,
    text: Option<String>
}

#[wasm_bindgen]
impl SliderBuilder {

    #[wasm_bindgen(constructor)]
    pub fn builder() -> SliderBuilder { 
        let x = None;
        let y = None;
        let opacity = None;
        let text = None;
        SliderBuilder { x, y, opacity, text }
    }

    pub fn build(&mut self) -> Slider {
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
        Slider { this, text, x, y, opacity }
    }

    pub fn x(mut self, x: f32) -> SliderBuilder {
        self.x = Some(x);
        self
    } 
    
    pub fn y(mut self, y: f32) -> SliderBuilder {
        self.y = Some(y);
        self
    } 
    
    pub fn opacity(mut self, opacity: f32) -> SliderBuilder {
        self.opacity = Some(opacity);
        self
    } 
    
    pub fn text(mut self, text: &str) -> SliderBuilder {
        let owned_text = text.to_owned();
        self.text = Some(owned_text);
        self
    } 
}