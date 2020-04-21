use crate::events::handler::Handler;
use std::rc::Rc;
use std::rc::Weak;
use crate::controls::visual_node::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use std::cell::RefCell;
use crate::events::mouse::*;

//#[derive(Copy)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    pub parent: Weak<dyn VisualNode>,
    pub children: Vec<Rc<dyn VisualNode>>,
}

impl Node {

    pub fn draw_children(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {
        for node in self.children.iter_mut() {
            web_sys::console::log_1(&"draw_children".into());
            let mut_node = Rc::get_mut(node).unwrap();
            mut_node.draw_children_(context, program, time);
        }
        for node in self.children.iter_mut() {
            let mut_node = Rc::get_mut(node).unwrap();
            mut_node.draw(context, program, time);
        }
    }

    pub fn propagate_events(&mut self, events: Rc<RefCell<Handler>>) {
        for node in self.children.iter_mut() {
            web_sys::console::log_1(&"propagate_events".into());
            let mut_node = Rc::get_mut(node).unwrap();
            mut_node.propagate_events_(Rc::clone(&events));
        }
        for node in self.children.iter_mut() {
            let mut_node = Rc::get_mut(node).unwrap();
            Node::send_event(Rc::clone(&events), mut_node);
        }
    }

    fn draw(gl: &WebGlRenderingContext, program: &WebGlProgram, node: &mut VisualNode, dt: f32) {
        node.draw(&gl, &program, dt);
    }

   fn send_event(events: Rc<RefCell<Handler>>,  node: &mut VisualNode) {
        let x = events.borrow().event.x;
        let y = events.borrow().event.y;
        let xy = node.position();
        if xy.0 < x as f32 && xy.2 > x as f32 && xy.1 < y as f32 && xy.3 > y as f32 {
            web_sys::console::log_1(&"sending event".into());
            let handled = node.event(&events.borrow().event);
            if handled {
                let mouse_event = Mouse::new(0, 0, MouseEvent::None);
                events.borrow_mut().set_event(mouse_event);
            }
        }
    }

    pub fn get_children(&self) -> &[Rc<dyn VisualNode>] {
        self.children.as_slice()
    }

    pub fn add_child(&mut self, node: Rc<dyn VisualNode>) {
        self.children.push(Rc::clone(&node));
    }

    pub fn set_parent(&mut self, node: Rc<dyn VisualNode>) {
        self.parent = Rc::downgrade(&node);
    }

    pub fn get_parent(&self) -> Weak<dyn VisualNode> {
        /*
        // Rust way of checking optional
        if let Some(parent) = &self.parent {  // Null check
            if let Some(parent) = parent.upgrade() { // Weak ref check
                return Some(Rc::new(parent.as_ref()));
            }
        }
        None
        */
        Weak::clone(&self.parent)
    }

    pub fn get_root(&self) -> Rc<dyn VisualNode> {
        let mut parent = self.get_parent().upgrade();
        let mut prev_parent : Option<Rc<dyn VisualNode>> = None;
        while parent.is_some() {
            match parent {
                Some(rc_parent) => { 
                    parent = rc_parent.get_node().get_parent().upgrade(); 
                    prev_parent = Some(rc_parent); 
                    
                },
                None => { return prev_parent.unwrap() }, 
            }
        }
        prev_parent.unwrap()
    }
}

#[cfg(test)]
mod tests {
 use super::*;

    #[test]
    fn test_get_root() {
        let parent = Node { x:0.0 , y:0.0, opacity:0.0, parent: Weak::new(), children: vec![]};
        let parent = Rc::new(parent);
        let child = Node { x:0.0 , y:0.0, opacity:1.0, parent: Rc::downgrade(&parent), children: vec![]};
        assert!(parent.opacity == child.get_root().opacity);
    } 
}
