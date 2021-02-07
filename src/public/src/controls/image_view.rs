use wasm_bindgen::prelude::*;

use uuid::Uuid;
use serde::*;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Serialize)]
pub struct ImageView {
    this: Uuid,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
    image: String,
}

#[wasm_bindgen]
pub struct ImageViewBuilder {
     x: Option<f32>,
     y: Option<f32>,
     width: Option<f32>,
     height: Option<f32>,
     translate_x: Option<f32>,
     translate_y: Option<f32>,
     opacity: Option<f32>,
     texture: Option<String>,
}

#[wasm_bindgen]
impl ImageViewBuilder {
    
    #[wasm_bindgen(constructor)]
    pub fn builder() -> ImageViewBuilder { 
        let x = None;
        let y = None;
        let translate_x = None;
        let translate_y = None;
        let opacity = None;
        let width = None;
        let height = None;
        let texture = None;
        ImageViewBuilder { x, y, translate_x, translate_y, opacity, width, height, texture }
    }

    pub fn build(&self) -> ImageView {
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
        let image:String = match &self.texture {
            Some(texture) => texture.to_string(),
            None => "None".to_owned() //TODO mandatory!
        };
        let this = Uuid::new_v4();
        ImageView { this, x, y, width, height, translate_x, translate_y, opacity, image }
    }

    pub fn x(mut self, x: f32) -> ImageViewBuilder {
        self.x = Some(x);
        self
    } 

    pub fn y(mut self, y: f32) -> ImageViewBuilder {
        self.y = Some(y);
        self
    } 

    pub fn translate_x(mut self, translate_x: f32) -> ImageViewBuilder {    
        self.translate_x = Some(translate_x);
        self
    } 

    pub fn  translate_y(mut self, translate_y: f32) -> ImageViewBuilder {
        self.translate_y = Some(translate_y);
        self
    } 

    pub fn opacity(mut self, opacity: f32) -> ImageViewBuilder {
        self.opacity = Some(opacity);
        self
    } 

    pub fn width(mut self, width: f32) -> ImageViewBuilder {
        self.width = Some(width);
        self
    } 

    pub fn height(mut self, height: f32) -> ImageViewBuilder {
        self.height = Some(height);
        self
    }

    pub fn image(mut self, texture: &str) -> ImageViewBuilder {
        self.texture = Some(texture.to_string());
        self
    }
    
}