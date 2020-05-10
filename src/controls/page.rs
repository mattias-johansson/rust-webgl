use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use crate::controls::button::*;
use std::rc::Weak;
use std::rc::Rc;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use std::any::Any;
use crate::events::mouse::*;
use std::cell::RefCell;
use crate::events::handler::Handler;
use uuid::Uuid;
use crate::render::texture_unit::*;
use crate::application::context::*;

pub struct Page {
    node: Node
}

impl VisualNode for Page {


    fn as_any(&self) -> &dyn Any {
        self
    }

    fn event_handler(&self, cx: &mut Context, message: &Event) -> bool{
        return false;
    }
}

impl Page {

    pub fn get(&mut self) -> &mut Page {
        self
    }
    
    pub fn new(cx: &mut Context) ->  Page {
        let x:f32 = 0.0;
        let y:f32 = 0.0;
        let translate_x = 0.0;
        let translate_y = 0.0;
        let opacity:f32 = 0.0;
        let width = 107.0;
        let height = 36.0;
        let texture = TextureUnit::None;
        let dirty = true;
        let uuid = Uuid::new_v4();
        let node = Node { uuid, x, y, width, height, translate_x, translate_y, opacity, texture, dirty };
        Page { node }
    }
}
