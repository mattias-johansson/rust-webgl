use wasm_bindgen::prelude::*;

use uuid::Uuid;

use uimsg::ListView;

#[wasm_bindgen]
pub struct ListViewBuilder {
     x: Option<f32>,
     y: Option<f32>,
     width: Option<f32>,
     height: Option<f32>,
     translate_x: Option<f32>,
     translate_y: Option<f32>,
     opacity: Option<f32>,
}

#[wasm_bindgen]
impl ListViewBuilder {
    
    #[wasm_bindgen(constructor)]
    pub fn builder() -> ListViewBuilder { 
        let x = None;
        let y = None;
        let translate_x = None;
        let translate_y = None;
        let opacity = None;
        let width = None;
        let height = None;
        ListViewBuilder { x, y, translate_x, translate_y, opacity, width, height }
    }

    pub fn build(&self) -> ListView {
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

        ListView::new(x, y, translate_x, translate_y, opacity, width, height)
    }

    pub fn x(mut self, x: f32) -> ListViewBuilder {
        self.x = Some(x);
        self
    } 

    pub fn y(mut self, y: f32) -> ListViewBuilder {
        self.y = Some(y);
        self
    } 

    pub fn translate_x(mut self, translate_x: f32) -> ListViewBuilder {    
        self.translate_x = Some(translate_x);
        self
    } 

    pub fn  translate_y(mut self, translate_y: f32) -> ListViewBuilder {
        self.translate_y = Some(translate_y);
        self
    } 

    pub fn opacity(mut self, opacity: f32) -> ListViewBuilder {
        self.opacity = Some(opacity);
        self
    } 

    pub fn width(mut self, width: f32) -> ListViewBuilder {
        self.width = Some(width);
        self
    } 

    pub fn height(mut self, height: f32) -> ListViewBuilder {
        self.height = Some(height);
        self
    }
}