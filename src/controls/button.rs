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
    node: Node,
    pressed: bool,
    animation: Option<Animation>
}

impl ButtonPrivate {
    pub fn new(cx: &mut Context, x: f32, y: f32, opacity: f32, parent: Weak<dyn VisualNode>) -> ButtonPrivate { 
        let width = 145.0;
        let height = 34.0;
        let translate_x = 0.0;
        let translate_y = 0.0;
        let height = 34.0;
        let texture = TextureUnit::Button;
        let dirty = true;
        let uuid = Uuid::new_v4();
        let node = Node { uuid, x, y, width, height, translate_x, translate_y, opacity, texture, dirty };
        let pressed = false;
        let animation = Option::None; 
        ButtonPrivate { node, pressed, animation } 
    }

    pub fn to_button_private(s: &dyn Any) -> Option<&ButtonPrivate>{
        if let Some(button) = s.downcast_ref::<ButtonPrivate>() {
            Some(&button)
        } else {
            None
        }
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
