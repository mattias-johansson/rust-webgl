use std::rc::Rc;

//#[derive(Copy)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    parent: Option<Rc<Node>>,
}

impl Node {

    pub fn get_parent(&self) -> Option<Rc<Node>> {
        if let Some(parent) = &self.parent {
            return Some(Rc::clone(&parent))            
        }
        None
    }

    pub fn get_root(&self) -> &Node {
        let mut parent = self.get_parent();
        while parent.is_some() {
            match parent {
                Some(rc_parent) => { parent = rc_parent.get_parent(); },
                None => { return self; }, 
            }
        }
        self
    }
}
