
use std::rc::Rc;
use std::rc::Weak;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use std::any::Any;
use crate::events::mouse::*;
use uuid::Uuid;
use crate::render::texture_unit::*;
use crate::application::context::*;

pub struct Container {
    node: Node
}

impl VisualNode for Container {

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool{
        return false;
    }
}

impl Container {

    pub fn get(&mut self) -> &mut Container {
        self
    }

    pub fn new(cx: &mut Context) ->  Container {
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
        Container { node }
    }
}
