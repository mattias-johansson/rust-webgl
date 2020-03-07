use std::any::Any;
use std::rc::Rc;
use std::rc::Weak;
use crate::rnd::texture_unit::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animator::*;

extern crate erased_serde;

pub struct ButtonPrivate {
    node: Node,
    pressed: bool,
    animator: Option<Animator>
}


impl ButtonPrivate {
    pub fn new(x: f32, y: f32, opacity: f32, parent: Weak<Node>) -> ButtonPrivate { 
        let children = vec![];
        let node = Node { x, y, opacity, parent, children };
        let pressed = false;
        let animator = Option::None; 
        ButtonPrivate { node, pressed, animator } 
    }

    pub fn to_button_private(s: &dyn Any) -> Option<&ButtonPrivate>{
        if let Some(toggle_button) = s.downcast_ref::<ButtonPrivate>() {
            Some(&toggle_button)
        } else {
            None
        }
    }
}


impl VisualNode for ButtonPrivate {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_parent(&mut self, node: Node) {
        self.node.set_parent(Rc::new(node));
    }

    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {
        if self.pressed {
            render(&context, &program, 145.0, 34.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::ButtonPressed);
        } else {
            render(&context, &program, 145.0, 34.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::Button);    
        }
    }

    fn event(&mut self, event: &Mouse) {
        web_sys::console::log_1(&"button handling event".into());   
        if event.event == MouseEvent::Up {
            self.pressed = false;
        } else if event.event == MouseEvent::Down {
            self.pressed = true; 
        }        
    }

   fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 145.0, self.node.y + 34.0)
    }
}
