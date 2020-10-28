

use uuid::Uuid;
use crate::application::context::*;
use crate::render::textures::*;

#[derive(Clone, Copy)]
pub struct Node {
    pub owner: Uuid,
    pub uuid: Uuid,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub opacity: f32,
    pub texture: Option<Uuid>,
    pub color: (f32,f32,f32),
    pub dirty: bool,
    pub clip: bool,
    pub end_clip: bool,
    pub text: bool,
//    pub callback: Option<fn(&mut Context) -> bool >,
}

impl Node {

    pub fn new(owner: Uuid, x: f32, y: f32, width: f32, height: f32) -> Node {
        Node { 
            owner: owner,
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
            dirty: true,
            clip: false,
            end_clip: false,
            text: false,
//            callback: None,
         }
    }

    pub fn create_texture(cx: &mut Context, image: &str) -> Uuid {
        let texture = Texture::new(image.to_string());
        let uuid = texture.id;
        cx.textures.textures.insert(uuid, texture);
        uuid
    }

    pub fn add_child(&self, cx: &mut Context, child: Uuid) {
        cx.add_child_to(self.uuid, child);
    }

    pub fn remove_child(&self, cx: &mut Context, child: Uuid) {
        cx.remove_child_from(self.uuid, child);
    }

    pub fn position(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + self.width, self.y + self.height)
    }
/*
    pub fn event_handler(&self, mut cx: &mut Context) -> bool {
        match self.callback {
            Some(callback) => {
                (callback)(&mut cx)
            },
            None => {
                false
            }
        }
    }
    */
}
