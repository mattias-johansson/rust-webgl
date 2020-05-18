use crate::application::context::Context;
use crate::events::handler::Handler;
use std::any::Any;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;
use crate::controls::node::Node;
use std::rc::Rc;
use std::cell::RefCell;

pub trait VisualNode {

    fn event_handler(&mut self, cx: &mut Context, event: &Event) -> bool;

    fn as_any(&mut self) -> &mut dyn Any;

}