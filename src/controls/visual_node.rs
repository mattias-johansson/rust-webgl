use crate::events::handler::Handler;
use std::any::Any;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;
use crate::controls::node::Node;
use std::rc::Rc;
use std::cell::RefCell;

pub trait VisualNode {

    fn draw_children_(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32);

    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32);

    fn propagate_events_(&mut self, events: Rc<RefCell<Handler>>);

    fn event(&mut self, event: &Mouse);

    fn position(&self) -> (f32, f32, f32, f32);

    fn set_parent(&mut self, node: Rc<dyn VisualNode>);

    fn add_child(&mut self, node: Rc<dyn VisualNode>);

    fn get_node(&self) -> &Node;

    fn as_any(&self) -> &dyn Any;

}