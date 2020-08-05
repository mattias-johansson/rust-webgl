
use std::rc::Rc;
use std::rc::Weak;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use std::any::Any;
use crate::events::mouse::*;
use uuid::Uuid;
use crate::render::texture_unit::*;
use crate::application::context::*;

#[derive(Clone)]
pub struct Container {
    this: Uuid
}

impl VisualNode for Container {

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

impl Container {

    pub fn get(&mut self) -> &mut Container {
        self
    }

    pub fn new(cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color:(f32,f32,f32), texture:TextureUnit) ->  Container {
        let x:f32 = x;
        let y:f32 = y;
        let translate_x = translate_x;
        let translate_y = translate_y;
        let opacity:f32 = opacity;
        let width = width;
        let height = height;
        let texture = None;
        let color = color;
        let dirty = true;
        let uuid = Uuid::new_v4();
        let parent = Uuid::new_v4();
        let node = Node { parent, uuid, x, y, width, height, translate_x, translate_y, opacity, texture, color, dirty };
        cx.nodes.push(node);
        Container { this: parent }
    }
}

pub struct ContainerBuilder {
     x: Option<f32>,
     y: Option<f32>,
     width: Option<f32>,
     height: Option<f32>,
     translate_x: Option<f32>,
     translate_y: Option<f32>,
     opacity: Option<f32>,
     texture: Option<TextureUnit>,
     color: Option<(f32,f32,f32)>,
}



impl ContainerBuilder {
    
    pub fn builder() -> ContainerBuilder { 
        let x = None;
        let y = None;
        let translate_x = None;
        let translate_y = None;
        let opacity = None;
        let width = None;
        let height = None;
        let color = None;
        let texture = None;
        ContainerBuilder { x, y, translate_x, translate_y, opacity, width, height, color, texture }
    }

    pub fn build(&self, cx: &mut Context) -> Container {
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
        let texture:TextureUnit = match self.texture {
            Some(texture) => texture,
            None => TextureUnit::None
        };
        let color:(f32,f32,f32) = match self.color {
            Some(color) => color,
            None => (0.0,0.0,0.0)
        };
        Container::new(cx, x, y, translate_x, translate_y, opacity, width, height, color, texture)
    }

    pub fn x(&mut self, x: f32) -> &mut ContainerBuilder {
        self.x = Some(x);
        self
    } 

    pub fn y(&mut self, y: f32) -> &mut ContainerBuilder {
        self.y = Some(y);
        self
    } 

    pub fn translate_x(&mut self, translate_x: f32) -> &mut ContainerBuilder {    
        self.translate_x = Some(translate_x);
        self
    } 

    pub fn  translate_y(&mut self, translate_y: f32) -> &mut ContainerBuilder {
        self.translate_y = Some(translate_y);
        self
    } 

    pub fn opacity(&mut self, opacity: f32) -> &mut ContainerBuilder {
        self.opacity = Some(opacity);
        self
    } 

    pub fn width(&mut self, width: f32) -> &mut ContainerBuilder {
        self.width = Some(width);
        self
    } 

    pub fn height(&mut self, height: f32) -> &mut ContainerBuilder {
        self.height = Some(height);
        self
    }

    pub fn color(&mut self, color: (f32,f32,f32)) -> &mut ContainerBuilder {
        self.color = Some(color);
        self
    }
}