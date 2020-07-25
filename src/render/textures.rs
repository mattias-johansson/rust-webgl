use std::collections::HashMap;

use web_sys::WebGlRenderingContext;
use std::rc::Rc;

use crate::render::lti::*;

pub struct Textures {
    textures: HashMap<String, (i32, u32)>,
    last_used_pos_u: u32,
    last_used_pos_i: i32,
}

impl Textures {

    pub fn new() -> Textures {
        Textures { textures: HashMap::default(), last_used_pos_u: 33988u64 as u32, last_used_pos_i: 4 as i32}
    }

    pub fn load_texture(&mut self, gl: Rc<WebGlRenderingContext>, texture :&str) -> i32 {
        let free_pos = self.get_free_pos();
        match free_pos {
            Some(free_pos) => {
                load_texture_image(gl, texture, free_pos.1);
                self.load_texture_internal(texture, free_pos);
                self.last_used_pos_i = self.last_used_pos_i +1;
                self.last_used_pos_u = self.last_used_pos_u +1;
                free_pos.0
            },
            None => {
                0 //TODO Handle no free textures
            } 
        }

    }

    pub fn get_texture_position(&self, texture :&str) -> Option<&(i32, u32)> {
        self.textures.get(texture)
    }

    fn load_texture_internal(&mut self, texture: &str, position: (i32, u32)) {
        self.textures.insert(texture.to_string(),position);
    }
    
    fn get_free_pos(&self) -> Option<(i32, u32)> {
        Some((self.last_used_pos_i+1, self.last_used_pos_u+1)) //TODO return none when out of space
    }
}