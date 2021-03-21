use uuid::Uuid;
use crate::application::context::*;
use crate::render::textures::*;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone, Copy)]
pub struct Node {
    pub owner: Uuid,
    pub uuid: Uuid,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
    texture: Option<Uuid>,
    color: (f32,f32,f32),
    dirty: bool,
    pub clip: bool,
    pub end_clip: bool,
    pub text: bool,
}

impl Node {


    pub fn create(owner: Uuid, uuid: Uuid , x: f32, y: f32, width: f32, height: f32, translate_x: f32, translate_y:f32, opacity:f32, texture: Option<Uuid>, color: (f32,f32,f32), dirty: bool, clip: bool, end_clip: bool, text: bool) -> Node {
        Node { 
            owner: owner,
            uuid: uuid, //Uuid::new_v4(), 
            x: x, 
            y: y, 
            width: width, 
            height: height, 
            translate_x: translate_x, 
            translate_y: translate_y, 
            opacity: opacity, 
            texture: texture,
            color: color,
            dirty: dirty,
            clip: clip,
            end_clip: end_clip,
            text: text,
         }
    }

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
        web_sys::console::debug_1(&"create_texture".into());
        match cx.textures.textures_by_string.get(image) {
            Some(uuid) => { 
                *uuid 
            },
            None => {
                web_sys::console::debug_1(&"creating texture:".into());
                let texture = Texture::new(image.to_string());
                let uuid = texture.id;
                cx.textures.textures.insert(uuid, texture);
                cx.textures.textures_by_string.insert(image.to_owned(), uuid);
                uuid
            }
        }
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

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn translate_x(&self) -> f32 {
        self.translate_x
    }

    pub fn translate_y(&self) -> f32 {
        self.translate_y
    }

    pub fn texture(&self) -> Option<Uuid> {
        self.texture
    }

    pub fn color(&self) -> (f32,f32,f32) {
        self.color
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_translate_x(&mut self, translate_x: f32) {
        self.translate_x = translate_x;
    }
    
    pub fn set_translate_y(&mut self, translate_y: f32) {
        self.translate_y = translate_y;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
    
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
    }

    /// Set the node's texture.
    pub fn set_texture(&mut self, texture: Option<Uuid>) {
        self.texture = texture;
    }
}
