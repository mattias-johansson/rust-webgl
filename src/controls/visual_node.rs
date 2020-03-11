use std::any::Any;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;
use crate::controls::node::Node;
use std::rc::Rc;

pub trait VisualNode {

    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32);

    fn event(&mut self, event: &Mouse);

    fn position(&self) -> (f32, f32, f32, f32);

    fn set_parent(&mut self, node: Rc<dyn VisualNode>);

    fn get_node(&self) -> &Node;

    fn as_any(&self) -> &dyn Any;

}