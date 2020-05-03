use crate::controls::node::*;

pub struct Context {
    pub nodes: Vec<Node>
}

impl Context {
    pub fn new() -> Context {
        let nodes = vec![];
        Context { nodes }
    }
}