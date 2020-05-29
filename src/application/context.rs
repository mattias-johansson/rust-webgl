use crate::animation::animation::Animation;
use crate::controls::node::*;
use crate::controls::visual_node::*;
use crate::events::mouse::*;
use uuid::Uuid;

pub struct Context {
    pub visual_nodes: Vec<Box<dyn VisualNode>>,
    pub nodes: Vec<Node>,
    pub animations: Vec<Animation>,
    pub events: Vec<Event>
}

impl Context {
    pub fn new() -> Context {
        let visual_nodes = vec![];
        let nodes = vec![];
        let animations = vec![];
        let events = vec![];
        Context { visual_nodes, nodes, animations, events }
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