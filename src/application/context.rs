use crate::controls::list_view::ListTraitHolder;
use crate::render::word::Word;
use crate::animation::animation::Animation;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::render::textures::*;
use crate::globals::messaging::*;

use web_sys::{Worker};

use uuid::Uuid;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Context {
    pub nodes: Vec<Node>,
    pub node_relations: HashMap<Uuid, Vec<Uuid>>,
    pub animations: Vec<Animation>,
    pub events: Vec<Event>,
    pub textures: Textures,
    pub root: Option<Uuid>,
    pub callbacks: HashMap<Uuid, Box<dyn FnMut(&mut Context) >>,
    pub fonts: Word,
    pub vertices: HashMap<Uuid, Vec<f32>>,
    pub dirty: Rc<RefCell<bool>>,
    pub messaging: Messaging,
    pub word: Word,
    pub list_traits: HashMap<Uuid, ListTraitHolder>,
}

impl Context {
    pub fn new(worker: Worker) -> Context {
        let nodes = vec![];
        let node_relations = HashMap::default();
        let animations = vec![];
        let events = vec![];
        let textures = Textures::new();
        let root: Option<Uuid> = None;
        let callbacks = HashMap::default();
        let fonts = Word::default();
        let vertices = HashMap::default();
        let dirty = Rc::new(RefCell::new(false));
        let messaging = Messaging { worker };
        let word = Word::default();
        let list_traits = HashMap::default();
        Context { nodes, node_relations, animations, events, textures, root, callbacks, fonts, vertices, dirty, messaging, word, list_traits }
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
        let mut dirty = self.dirty.borrow_mut();
        *dirty = true;
        let mut children : Option<&mut Vec<Uuid>> = self.node_relations.get_mut(&parent);
        match &mut children {
            Some(children) => {
                if !children.contains(&child) {
                    children.push(child);
                }
            },
            None => {
                web_sys::console::log_1(&"New vec".into());
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
                let mut dirty = self.dirty.borrow_mut();
                *dirty = true;
            },
            None => ()
        }
    }

    pub fn remove_all_child_from(&mut self, parent: Uuid) {
        let mut children : Option<&mut Vec<Uuid>> = self.node_relations.get_mut(&parent);
        match &mut children {
            Some(children) => {
                children.clear();
                let mut dirty = self.dirty.borrow_mut();
                *dirty = true;
            },
            None => ()
        }
    }
}
