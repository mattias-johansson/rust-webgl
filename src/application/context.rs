use crate::render::word::Word;
use crate::application::core_app::CoreApp;
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
    pub cb: Callbacks,
    pub fonts: Word,
    pub vertices: HashMap<Uuid, Vec<f32>>,
    pub dirty: Rc<RefCell<bool>>,
    pub messaging: Messaging,
    pub word: Word,

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
        let cb = Callbacks::new();
        let fonts = Word::default();
        let vertices = HashMap::default();
        let dirty = Rc::new(RefCell::new(false));
        let messaging = Messaging { worker };
        let word = Word::default();
        Context { nodes, node_relations, animations, events, textures, root, callbacks, cb, fonts, vertices, dirty, messaging, word }
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


#[derive(Clone)]
pub struct Callbacks {
    triggered_callbacks : Vec<(Uuid, Event)>,
    subscribers: HashMap<Uuid, Vec<fn(&mut Context, &mut CoreApp, Event)>>
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks { triggered_callbacks: vec![], subscribers: HashMap::default() }
    }

    pub fn add_trigged(&mut self, source: Uuid, event: Event) {
        self.triggered_callbacks.push((source, event));
    }

    pub fn clear_triggered(&mut self) {
        self.triggered_callbacks.clear();
    }

    pub fn add_subscriber(&mut self, source: Uuid, target: fn(&mut Context, &mut CoreApp, Event)) {
        let mut callbacks = self.subscribers.get_mut(&source);
        match &mut callbacks {
            Some(cb) => {
                cb.push(target);
            },
            None => {
                let mut callbacks = vec![];
                callbacks.push(target);
                self.subscribers.insert(source, callbacks);

            }
        }
    }

    pub fn trigger_callbacks(&self, cx: &mut Context, core_app: &mut CoreApp) {
        for triggerd in &self.triggered_callbacks {
            let targets = self.subscribers.get(&triggerd.0);
            match targets {
                Some(targets) => {
                    for target in targets {
                        (target)(cx, core_app, triggerd.1);
                        web_sys::console::log_1(&"triggered".into());

                    }
                },
                None => {
                }
            }
        }
    }
}
