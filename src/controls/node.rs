use crate::events::handler::Handler;
use crate::events::mouse::*;
use crate::render::texture_unit::*;
use crate::controls::visual_node::*;
use std::rc::Rc;
use std::rc::Weak;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use std::cell::RefCell;
use uuid::Uuid;
use crate::application::context::*;

//#[derive(Copy)]
pub struct Node {
    pub uuid: Uuid,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub opacity: f32,
    pub texture: TextureUnit,
    pub dirty: bool,
}

impl Node {

    pub fn new(cx: &mut Context, x: f32, y: f32, width: f32, height: f32) -> Node {
        Node {uuid: Uuid::new_v4(), 
            x: x, 
            y: y, 
            width: width, 
            height: height, 
            translate_x: 0.0, 
            translate_y: 0.0, 
            opacity: 0.0, 
            texture: TextureUnit::None,
            dirty: true
         }
    }
    
    pub fn position(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + self.width, self.y + self.height)
    }
}
