use crate::controls::node::*;
use crate::controls::visual_node::*;
use uuid::Uuid;

pub struct Context {
    pub nodes: Vec<Node>,
    pub visual_nodes: Vec<Box<dyn VisualNode>>
}

impl Context {
    pub fn new() -> Context {
        let nodes = vec![];
        let visual_nodes = vec![];
        Context { nodes, visual_nodes }
    }

    pub fn get_node(&mut self, uuid: Uuid) -> Option<&mut Node> {
        let nodes: &mut Vec<Node> = self.nodes.as_mut(); 
        for node in nodes {
            if uuid == node.uuid {
                return Some(node);
            }
        }
        None
    }

    pub fn get_owner(&mut self, node: Uuid) -> &mut Box<dyn VisualNode> {
        let visual_node = self.visual_nodes.first_mut();
        let visual_node = visual_node.unwrap();
        visual_node
    }
}