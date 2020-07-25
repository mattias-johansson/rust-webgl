
use crate::render::texture_unit::*;
use uuid::Uuid;
use crate::application::context::*;

#[derive(Clone, Copy)]
pub struct Node {
    pub parent: Uuid,
    pub uuid: Uuid,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub opacity: f32,
    pub texture: Option<u64>,
    pub color: (f32,f32,f32),
    pub dirty: bool,
}

impl Node {

    pub fn new(parent: Uuid, cx: &mut Context, x: f32, y: f32, width: f32, height: f32) -> Node {
        Node { 
            parent: parent,
            uuid: Uuid::new_v4(), 
            x: x, 
            y: y, 
            width: width, 
            height: height, 
            translate_x: 0.0, 
            translate_y: 0.0, 
            opacity: 1.0, 
            texture: None,
            color: (0.0, 0.0, 0.0),
            dirty: true
         }
    }
    
    pub fn position(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + self.width, self.y + self.height)
    }
}
