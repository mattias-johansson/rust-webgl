use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;
use crate::controls::node::Node;
use std::rc::Rc;

extern crate erased_serde;

pub trait VisualNode {

    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32);

    fn event(&mut self, event: &Mouse);

    fn position(&self) -> (f32, f32, f32, f32);

    fn set_parent(&mut self, node: Node);

}