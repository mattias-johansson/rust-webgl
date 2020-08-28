use crate::animation::animation::Animation;
use crate::controls::node::*;
use crate::controls::visual_node::*;
use crate::events::mouse::*;
use crate::render::textures::*;
use uuid::Uuid;
use std::collections::HashMap;

pub struct Context {
    pub visual_nodes: Vec<Box<dyn VisualNode>>,
    pub nodes: Vec<Node>,
    pub node_relations: HashMap<Uuid, Vec<Uuid>>,
    pub animations: Vec<Animation>,
    pub events: Vec<Event>,
    pub textures: Textures,
    pub root: Option<Uuid>,
}

impl Context {
    pub fn new() -> Context {
        let visual_nodes = vec![];
        let nodes = vec![];
        let node_relations = HashMap::default();
        let animations = vec![];
        let events = vec![];
        let textures = Textures::new();
        let root: Option<Uuid> = None;
        Context { visual_nodes, nodes, node_relations, animations, events, textures, root }
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

    pub fn get_node_unmut(&self, uuid: Uuid) -> Option<&Node> {
        for node in self.nodes.as_slice() {
            if uuid == node.uuid {
                return Some(&node);
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

    pub fn get_node_uuid(&mut self, uuid: Uuid) -> Option<Uuid> {
        for node in &self.nodes {
            if uuid == node.uuid {
                return Some(node.uuid);
            }
        }
        None
    }

    pub fn add_child_to(&mut self, parent: Uuid, child: Uuid) {
        let mut children : Option<&mut Vec<Uuid>> = self.node_relations.get_mut(&parent);
        match &mut children {
            Some(children) => {
                children.push(child);
            },
            None => {
                let mut children = Vec::new();
                children.push(child);
                self.node_relations.insert(parent, children);
            }
        }
    }

    pub fn remove_child_from(&mut self, parent: Uuid, to_remove: Uuid) {
        let mut children : Option<&mut Vec<Uuid>> = self.node_relations.get_mut(&parent);
        match &mut children {
            Some(children) => {
                children.retain(|&x| x != to_remove);
            },
            None => ()
        }
    }
}