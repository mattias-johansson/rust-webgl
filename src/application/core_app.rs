
use uuid::Uuid;
use crate::controls::visual_node::VisualNode;

pub struct CoreApp {
    pub visual_nodes: Vec<Box <dyn VisualNode>>
}

impl CoreApp {
    
    pub fn get_visual_node(&mut self, uuid: Uuid) -> Option<&mut Box<dyn VisualNode>> {
        let visual_nodes: &mut Vec<Box<dyn VisualNode>> = self.visual_nodes.as_mut(); 
        for visual_node in visual_nodes {
            if uuid == visual_node.get_uuid() {
                return Some(visual_node);
            }
        }
        None
    }

}