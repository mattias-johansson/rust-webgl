use std::any::Any;
use crate::render::texture_unit::*;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::application::context::*;
use uuid::Uuid;

pub struct ButtonPrivate {
    this: Uuid,
    node_uuid: Uuid,
    pressed: bool,
    animation: Option<Animation>,
    pub closure: Option<Box<dyn FnMut(&mut Context) >>
}

impl ButtonPrivate {
    pub fn new(cx: &mut Context, x: f32, y: f32, opacity: f32) -> ButtonPrivate { 
        let this = Uuid::new_v4();
        let pressed = false;
        let animation = Option::None; 
        let node = ButtonPrivate::create(this, cx, x, y, opacity);
        let node_uuid = node.uuid;
        cx.nodes.push(node);
        let closure = None;
        ButtonPrivate { this, node_uuid, pressed, animation, closure } 
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

    fn event_handler(&mut self, mut cx: &mut Context, message: &Event) -> bool {
        web_sys::console::log_1(&"got event".into());
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                    self.pressed = false;
//                    self.on_button_pressed(cx);
                    if self.closure.is_some() {
                        let callback : &mut Box<dyn FnMut(&mut Context)> = self.closure.as_mut().unwrap();
                        web_sys::console::log_1(&"Calling callback".into());
                        callback(&mut cx);
                    }
                }else if event.event == MouseEvent::Down {
                    self.pressed = true; 
                }      
                return true;
            },
            Event::Message(message) => {
                match message {
//                    Message::AnimationEnded(Uuid) => self.on_animation_ended(cx),
                    _ => ()
                }
            }
        }
        web_sys::console::log_1(&"false".into());
        return false;
    }

    fn get_uuid(&self) -> uuid::Uuid { 
        self.this
    }
}
