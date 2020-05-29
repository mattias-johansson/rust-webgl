use std::any::Any;
use std::rc::Rc;
use crate::render::texture_unit::*;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::application::context::*;
use uuid::Uuid;
use crate::application::*;

pub struct ButtonPrivate {
    this: Uuid,
    node_uuid: Uuid,
    pressed: bool,
    animation: Option<Animation>
}

impl ButtonPrivate {
    pub fn new(cx: &mut Context, x: f32, y: f32, opacity: f32) -> ButtonPrivate { 
        let this = Uuid::new_v4();
        web_sys::console::log_1(&"ButtonPrivate".into());
        let pressed = false;
        let animation = Option::None; 
        web_sys::console::log_1(&"ButtonPrivate".into());
        let node = ButtonPrivate::create(this, cx, x, y, opacity);
        let node_uuid = node.uuid;
        web_sys::console::log_1(&"ButtonPrivate".into());
        cx.nodes.push(node);
        ButtonPrivate { this, node_uuid, pressed, animation } 
    }

    pub fn to_button_private(s: &dyn Any) -> Option<&ButtonPrivate>{
        if let Some(button) = s.downcast_ref::<ButtonPrivate>() {
            Some(&button)
        } else {
            None
        }
    }

    pub fn create(this: Uuid, cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width =  145.0;
        let height = 34.0;
        let texture = TextureUnit::Button;
        let mut node = Node::new(this, cx, x, y, width, height);
        node.texture = texture;
        node
    }

}


impl VisualNode for ButtonPrivate {

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool {
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                    self.pressed = false;
                     return true;
                } else if event.event == MouseEvent::Down {
                    self.pressed = true; 
                    return true;
                }      
            },
            _ => ()
        }
        return false;
    }

    fn get_uuid(&self) -> uuid::Uuid { 
        self.this
    }
}
