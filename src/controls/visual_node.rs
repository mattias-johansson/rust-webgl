use uuid::Uuid;
use crate::application::context::Context;
use std::any::Any;
use crate::events::mouse::*;

pub trait VisualNode {

    fn event_handler(&mut self, cx: &mut Context, event: &Event) -> bool;

    fn as_any(&mut self) -> &mut dyn Any;

    fn get_uuid(&self) -> Uuid;

}
/*
pub trait CloneVisualNode {

    fn clone_visual_node(&self) -> Box<dyn VisualNode>;

}

impl<T> CloneVisualNode for T
    where T: VisualNode + Clone + 'static {
    fn clone_visual_node(&self) -> Box<dyn VisualNode> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn VisualNode> {
    fn clone(&self) -> Self {
        self.clone_visual_node()
    }

}
*/
