use uuid::Uuid;
use crate::application::context::Context;
use std::any::Any;
use crate::events::mouse::*;

pub trait VisualNode : VisualNodeClone {

    fn event_handler(&mut self, cx: &mut Context, event: &Event) -> bool;

    fn as_any(&mut self) -> &mut dyn Any;

    fn get_uuid(&self) -> Uuid;

    fn get_node_uuid(&self) -> Uuid;

}

pub trait VisualNodeClone {
    fn clone_box(&self) -> Box< dyn VisualNode>;
}

impl<T: 'static + VisualNode + Clone> VisualNodeClone for T {
    fn clone_box(&self) -> Box<dyn VisualNode> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn VisualNode> {
    fn clone(&self) -> Box<dyn VisualNode> {
        self.clone_box()
    }
}