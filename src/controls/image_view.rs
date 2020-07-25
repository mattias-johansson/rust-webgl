
use std::rc::Rc;
use std::rc::Weak;
use std::any::Any;
use uuid::Uuid;
use crate::application::context::*;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::render::texture_unit::*;

#[derive(Clone)]
pub struct ImageView {
    this: Uuid,
    pub node: Node,
}

impl VisualNode for ImageView {

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool{
        return false;
    }

    fn get_uuid(&self) -> uuid::Uuid { 
        self.this
    }
}

impl ImageView {

    pub fn get(&mut self) -> &mut ImageView {
        self
    }

    pub fn new(cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color:(f32,f32,f32), texture: String) ->  ImageView {
        let x:f32 = x;
        let y:f32 = y;
        let translate_x = translate_x;
        let translate_y = translate_y;
        let opacity:f32 = opacity;
        let width = width;
        let height = height;
        let texture = Some(texture);
        let color = color;
        let dirty = true;
        let uuid = Uuid::new_v4();
        let parent = Uuid::new_v4();
        let node = Node { parent, uuid, x, y, width, height, translate_x, translate_y, opacity, texture, color, dirty };
        cx.nodes.push(node);
        ImageView { this: parent, node: node }
    }
}

pub struct ImageViewBuilder {
     x: Option<f32>,
     y: Option<f32>,
     width: Option<f32>,
     height: Option<f32>,
     translate_x: Option<f32>,
     translate_y: Option<f32>,
     opacity: Option<f32>,
     texture: Option<String>,
     color: Option<(f32,f32,f32)>,
}



impl ImageViewBuilder {
    
    pub fn builder() -> ImageViewBuilder { 
        let x = None;
        let y = None;
        let translate_x = None;
        let translate_y = None;
        let opacity = None;
        let width = None;
        let height = None;
        let color = None;
        let texture = None;
        ImageViewBuilder { x, y, translate_x, translate_y, opacity, width, height, color, texture }
    }

    pub fn build(&self, cx: &mut Context) -> ImageView {
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
        let texture:String = match &self.texture {
            Some(texture) => texture,
            None => "None".to_string() //TODO mandatory!
        };
        let color:(f32,f32,f32) = match self.color {
            Some(color) => color,
            None => (0.0,0.0,0.0)
        };
        ImageView::new(cx, x, y, translate_x, translate_y, opacity, width, height, color, texture)
    }

    pub fn x(&mut self, x: f32) -> &mut ImageViewBuilder {
        self.x = Some(x);
        self
    } 

    pub fn y(&mut self, y: f32) -> &mut ImageViewBuilder {
        self.y = Some(y);
        self
    } 

    pub fn translate_x(&mut self, translate_x: f32) -> &mut ImageViewBuilder {    
        self.translate_x = Some(translate_x);
        self
    } 

    pub fn  translate_y(&mut self, translate_y: f32) -> &mut ImageViewBuilder {
        self.translate_y = Some(translate_y);
        self
    } 

    pub fn opacity(&mut self, opacity: f32) -> &mut ImageViewBuilder {
        self.opacity = Some(opacity);
        self
    } 

    pub fn width(&mut self, width: f32) -> &mut ImageViewBuilder {
        self.width = Some(width);
        self
    } 

    pub fn height(&mut self, height: f32) -> &mut ImageViewBuilder {
        self.height = Some(height);
        self
    }

    pub fn color(&mut self, color: (f32,f32,f32)) -> &mut ImageViewBuilder {
        self.color = Some(color);
        self
    }

    pub fn image(&mut self, texture: &str) -> &mut ImageViewBuilder {
        self.texture = Some(texture.to_string());
        self
    }
    
}