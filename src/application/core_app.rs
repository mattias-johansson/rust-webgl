
use std::collections::HashMap;
use uuid::Uuid;
use crate::controls::visual_node::VisualNode;

pub struct CoreApp {
    pub visual_nodes: Vec<Box <dyn VisualNode>>,
    pub names: HashMap<String, Uuid>
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

    pub fn get_visual_node_from_name(&mut self, name: &str) -> Option<&mut Box<dyn VisualNode>> {
        let thing = self.names.get(name);
        let uuid : Uuid = *thing.unwrap_or(&Uuid::new_v4());
        self.get_visual_node(uuid)
    }
}