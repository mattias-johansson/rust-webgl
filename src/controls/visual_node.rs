use uuid::Uuid;
use crate::application::context::Context;
use std::any::Any;
use crate::events::mouse::*;

pub trait VisualNode {

    fn event_handler(&mut self, cx: &mut Context, event: &Event) -> bool;

    fn as_any(&mut self) -> &mut dyn Any;

    fn get_uuid(&self) -> Uuid;

    fn get_node_uuid(&self) -> Uuid;

}
