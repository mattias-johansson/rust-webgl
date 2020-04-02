use std::rc::Rc;
use std::rc::Weak;
use crate::controls::visual_node::*;

//#[derive(Copy)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    pub parent: Weak<dyn VisualNode>,
    pub children: Vec<Rc<dyn VisualNode>>,
}

impl Node {

    pub fn get_children(&self) -> Vec<Rc<dyn VisualNode>> {
        self.children.as_slice().to_owned()
    }

    pub fn add_child(&mut self, node: Rc<dyn VisualNode>) {
        self.children.push(Rc::clone(&node));
    }

    pub fn set_parent(&mut self, node: Rc<dyn VisualNode>) {
        self.parent = Rc::downgrade(&node);
    }

    pub fn get_parent(&self) -> Weak<dyn VisualNode> {
        /*
        // Rust way of checking optional
        if let Some(parent) = &self.parent {  // Null check
            if let Some(parent) = parent.upgrade() { // Weak ref check
                return Some(Rc::new(parent.as_ref()));
            }
        }
        None
        */
        Weak::clone(&self.parent)
    }

    pub fn get_root(&self) -> Rc<dyn VisualNode> {
        let mut parent = self.get_parent().upgrade();
        let mut prev_parent : Option<Rc<dyn VisualNode>> = None;
        while parent.is_some() {
            match parent {
                Some(rc_parent) => { 
                    parent = rc_parent.get_node().get_parent().upgrade(); 
                    prev_parent = Some(rc_parent); 
                    
                },
                None => { return prev_parent.unwrap() }, 
            }
        }
        prev_parent.unwrap()
    }
}

#[cfg(test)]
mod tests {
 use super::*;

    #[test]
    fn test_get_root() {
        let parent = Node { x:0.0 , y:0.0, opacity:0.0, parent: Weak::new(), children: vec![]};
        let parent = Rc::new(parent);
        let child = Node { x:0.0 , y:0.0, opacity:1.0, parent: Rc::downgrade(&parent), children: vec![]};
        assert!(parent.opacity == child.get_root().opacity);
    } 
}
