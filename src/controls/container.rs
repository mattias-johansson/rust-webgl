
use std::rc::Rc;
use std::rc::Weak;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use crate::controls::button::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use std::any::Any;
use crate::events::mouse::*;
use std::cell::RefCell;
use crate::events::handler::Handler;

pub struct Container {
    node: Node
}

impl VisualNode for Container {

    fn draw_children_(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {
        self.node.draw_children(context, program, time);
    }

    fn propagate_events_(&mut self, events: Rc<RefCell<Handler>>) {
        self.node.propagate_events(events);
    }

    fn get_node(&self) -> &Node {
        &self.node
    }

    fn add_child(&mut self, node: Rc<VisualNode>) {
        self.node.add_child(node);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_parent(&mut self, node: Rc<VisualNode>) {
        self.node.set_parent(node);
    }

    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {

    }

    fn event(&mut self, event: &Mouse)  -> bool {
        false
    }

    fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 107.0, self.node.y + 36.0)
    }
}

impl Container {

    pub fn get(&mut self) -> &mut Container {
        self
    }

    pub fn new() ->  Container {
        let children = vec![];
        let x:f32 = 0.0;
        let y:f32 = 0.0;
        let opacity:f32 = 0.0;
        let parent : Weak<ButtonPrivate> = Weak::new();
        let node = Node { x, y, opacity, parent, children };
        Container { node }
    }
}
