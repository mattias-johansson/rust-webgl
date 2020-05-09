use crate::controls::node::*;
use crate::controls::visual_node::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct Context {
    pub nodes: Vec<Node>
}

impl Context {
    pub fn new() -> Context {
        let nodes = vec![];
        Context { nodes }
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
}