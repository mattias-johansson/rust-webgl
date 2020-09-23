use crate::application::core_app::CoreApp;
use crate::animation::animation::Animation;
use crate::controls::node::*;
use crate::controls::visual_node::*;
use crate::events::mouse::*;
use crate::render::textures::*;
use uuid::Uuid;
use std::collections::HashMap;

pub struct Context {
    pub nodes: Vec<Node>,
    pub node_relations: HashMap<Uuid, Vec<Uuid>>,
    pub animations: Vec<Animation>,
    pub events: Vec<Event>,
    pub textures: Textures,
    pub root: Option<Uuid>,
    pub callbacks: HashMap<Uuid, Box<dyn FnMut(&mut Context) >>,
    pub cb: Callbacks,
//    pub visual_nodes: Vec<Box<dyn VisualNode>>

}

impl Context {
    pub fn new() -> Context {
        let nodes = vec![];
        let node_relations = HashMap::default();
        let animations = vec![];
        let events = vec![];
        let textures = Textures::new();
        let root: Option<Uuid> = None;
        let callbacks = HashMap::default();
        let cb = Callbacks::new();
//        let visual_nodes = vec![];
        Context { nodes, node_relations, animations, events, textures, root, callbacks, cb }
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
/*
    pub fn get_visual_node(&mut self, uuid: Uuid) -> Option<&mut Box<dyn VisualNode>> {
        let visual_nodes: &mut Vec<Box<dyn VisualNode>> = self.visual_nodes.as_mut(); 
        for visual_node in visual_nodes {
            if uuid == visual_node.get_uuid() {
                return Some(visual_node);
            }
        }
        None
    }
*/
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


#[derive(Clone)]
pub struct Callbacks {
    triggered_callbacks : Vec<Uuid>,
    subscribers: HashMap<Uuid, Vec<fn(&mut Context, &mut CoreApp)>>
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks { triggered_callbacks: vec![], subscribers: HashMap::default() }
    }

    pub fn add_trigged(&mut self, source: Uuid) {
        web_sys::console::log_1(&"added !!!!!!!!!!!!!!!!!!!!".into());
        self.triggered_callbacks.push(source);
    }

    pub fn clear_triggered(&mut self) {
        self.triggered_callbacks.clear();
    }

    pub fn add_subscriber(&mut self, source: Uuid, target: fn(&mut Context, &mut CoreApp)) {
        web_sys::console::log_1(&"add_subscriber".into());
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
            web_sys::console::log_1(&"trigger_callbacks!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".into());
            let targets = self.subscribers.get(&triggerd);
            match targets {
                Some(targets) => {

                    web_sys::console::log_1(&"Has targets !!!!!!!!!!!!!!!!!!!!!!!!!!!!!".into());
                    for target in targets {
                        (target)(cx, core_app);
                        web_sys::console::log_1(&"triggered".into());

                    }
                },
                None => {
                    web_sys::console::log_1(&"!!!!!!!!!!!!!!!!!!!!!!!!!!!!! NO targets ------------".into());
                }
            }
        }
    }
}
