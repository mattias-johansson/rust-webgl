use crate::application::context::Context;
use crate::events::handler::Handler;
use std::any::Any;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;
use crate::controls::node::Node;
use std::rc::Rc;
use std::cell::RefCell;

pub trait VisualNode {

    fn event_handler(&self, cx: &mut Context, event: &Event);

    fn as_any(&self) -> &dyn Any;

}