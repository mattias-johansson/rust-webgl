use crate::animation::animation::Animation;
use crate::controls::node::*;
use crate::controls::visual_node::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct Context {
    pub nodes: Vec<Node>,
    pub animations: Vec<Animation>
}

impl Context {
    pub fn new() -> Context {
        let nodes = vec![];
        let animations = vec![];
        Context { nodes, animations }
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

    pub fn get_animation(&mut self, uuid: Uuid) -> Option<&mut Animation> {
        let animations: &mut Vec<Animation> = self.animations.as_mut(); 
        for animation in animations {
            if uuid == animation.uuid {
                return Some(animation);
            }
        }
        None
    }
}