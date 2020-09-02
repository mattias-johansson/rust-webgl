use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use std::any::Any;
use std::rc::Rc;
use crate::events::mouse::*;
use uuid::Uuid;
use crate::render::texture_unit::*;
use crate::application::context::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Page {
    this: Uuid,
    pub node: Uuid
}

impl VisualNode for Page {

    fn get_node_uuid(&self) -> Uuid {
        self.node
    }

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
        let texture = None;
        let color = (0.0,0.0,0.0);
        let dirty = true;
        let clip = false;
        let end_clip = false;
        let uuid = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let node = Node { owner, uuid, x, y, width, height, translate_x, translate_y, opacity, texture, color, dirty, clip, end_clip};
        let node_uuid = node.uuid;
        cx.nodes.push(node);
        Page { this: owner, node: node_uuid }
    }
}
