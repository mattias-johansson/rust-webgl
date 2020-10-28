use uuid::Uuid;
use std::collections::HashMap;

use web_sys::WebGlRenderingContext;
use std::rc::Rc;

use crate::render::lti::*;

pub struct Textures {

    pub textures_by_string: HashMap<String, Uuid>,
    pub textures: HashMap<Uuid, Texture>,
    last_used_pos_u: u32,
    last_used_pos_i: i32,
}

pub struct Texture {
    pub id: Uuid,
    pub src: String,
    pub location: i32,
    pub other: u32,
}

impl Texture {

    pub fn new(src: String) -> Texture {
        let id = Uuid::new_v4();
        let location = -1;
        let other = 0;
        Texture { id, src, location, other }
    }
}

impl Textures {

    pub fn new() -> Textures {
        Textures { textures_by_string: HashMap::default(), textures: HashMap::default(), last_used_pos_u: 33984u64 as u32, last_used_pos_i: 0 as i32}
    }

    pub fn load_texture(&mut self, gl: Rc<WebGlRenderingContext>, texture_id: &Uuid) -> i32 {
        let free_pos = self.get_free_pos();
        let texture = self.textures.get_mut(texture_id);
        match texture {
            Some(texture) => {
                if texture.location ==  -1 {
                    match free_pos {
                        Some(free_pos) => {
                            load_texture_image(gl, texture.src.as_str(), free_pos.1);
                            self.last_used_pos_i = self.last_used_pos_i +1;
                            self.last_used_pos_u = self.last_used_pos_u +1;
                            texture.location = free_pos.0;
                            texture.location
                        },
                        None => {
                            0 //TODO Handle no free textures
                        } 
                    }
                }  else {
                    web_sys::console::log_1(&"texture.location".into());
                    web_sys::console::log_1(&texture.location.to_string().into());
                    texture.location
                }
            },
            None => {
                0 //TODO Propper error handling
            }
        }
    }

    pub fn get_texture_position(&self, texture: &Uuid) -> Option<&Texture> {
        self.textures.get(texture)
    }
    
    fn get_free_pos(&self) -> Option<(i32, u32)> {
        Some((self.last_used_pos_i+1, self.last_used_pos_u+1)) //TODO return none when out of space
    }
}