use std::rc::Rc;

//#[derive(Copy)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    pub parent: Option<Rc<Node>>,
    pub children: Vec<Rc<Node>>,
}

impl Node {

    pub fn add_child(&mut self, node: Rc<Node>) {
        self.children.push(Rc::clone(&node));
    }

    pub fn set_parent(&mut self, node: Node) {
        self.parent = Some(Rc::new(node));
    }

    pub fn get_parent(&self) -> Option<Rc<&Node>> {
        // Rust way of checking optional
        if let Some(parent) = &self.parent {
            return Some(Rc::new(parent.as_ref()));
        }
        None
    }

    pub fn get_root(&self) -> &Node {
        let mut parent = self.get_parent();
        let mut prev_parent = Rc::new(self);
        while parent.is_some() {
            match parent {
                Some(rc_parent) => { 
                    parent = rc_parent.get_parent(); 
                    prev_parent = rc_parent; 
                },
                None => { return prev_parent.as_ref(); }, 
            }
        }
        prev_parent.as_ref()
    }
}

#[cfg(test)]
mod tests {
 use super::*;

    #[test]
    fn test_get_root() {
        let parent = Node { x:0.0 , y:0.0, opacity:0.0, parent: None, children: vec![]};
        let parent = Rc::new(parent);
        let child = Node { x:0.0 , y:0.0, opacity:1.0, parent: Some(Rc::clone(&parent)), children: vec![]};
        assert!(parent.opacity == child.get_root().opacity);
    } 
}
