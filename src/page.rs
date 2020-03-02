

use crate::controls::draw::Draw;
use crate::controls::node::*;

pub struct Page {
    node: Node,
    child: Option<Box<dyn Draw>>,
}

impl Page {

    pub fn get(&mut self) -> &mut Page {
        self
    }

    pub fn setChild(&mut self, node: Box<dyn Draw>) {
        self.child = Some(node);
    } 

    pub fn new() ->  Page {
        let child = None;
        let children = vec![];
        let x:f32 = 0.0;
        let y:f32 = 0.0;
        let opacity:f32 = 0.0;
        let parent = None;
        let node = Node { x, y, opacity, parent, children };
        Page { node, child }
    }

    pub fn get_child(&mut self) -> &mut Option<Box<dyn Draw>> {
        &mut self.child
    }
}


//Type for reference to mutable slice of Box<dyn Draw>
// &mut [Box<dyn Draw>]
/*
impl ClickHandler<MouseEvent> for Container {
    fn do_things(&mut self, event : MouseEvent) {
        self.node_tree.push(Box::new(ButtonPrivate::new (10.0, 90.0, 0.0, None)));
    }
}
*/