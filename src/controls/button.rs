use std::any::Any;
use std::rc::Rc;
use std::rc::Weak;
use crate::render::texture_unit::*;
use crate::render::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::application::context::*;
use std::cell::RefCell;
use crate::events::handler::Handler;
use uuid::Uuid;
use crate::application::*;

pub struct ButtonPrivate {
    node_uuid: Uuid,
    pressed: bool,
    animation: Option<Animation>
}

impl ButtonPrivate {
    pub fn new(cx: &mut Context, x: f32, y: f32, opacity: f32) -> ButtonPrivate { 
        let pressed = false;
        let animation = Option::None; 
        let node = ButtonPrivate::create(cx, x, y, opacity);
        let node_uuid = node.uuid;
        
        cx.nodes.push(node);
        ButtonPrivate { node_uuid, pressed, animation } 
    }

    pub fn to_button_private(s: &dyn Any) -> Option<&ButtonPrivate>{
        if let Some(button) = s.downcast_ref::<ButtonPrivate>() {
            Some(&button)
        } else {
            None
        }
    }

    pub fn create(cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width =  145.0;
        let height = 34.0;
        let texture = TextureUnit::Button;
        let mut node = Node::new(cx, x, y, width, height);
        node.texture = texture;
        node
    }

}


impl VisualNode for ButtonPrivate {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn event_handler(&self, cx: &mut Context, message: &Event) -> bool {
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
            //        self.pressed = false;
                     return true;
                } else if event.event == MouseEvent::Down {
              //      self.pressed = true; 
                    return true;
                }      
            },
            _ => ()
        }
        return false;
    }
/*
   fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 145.0, self.node.y + 34.0)
    }
*/
}
