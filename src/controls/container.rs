
use std::rc::Rc;
use std::rc::Weak;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;

pub struct Container {
    node: Node,
    node_tree: Vec<Box<dyn VisualNode>>,
}

impl Container {

    fn add_parent(&mut self, node: Node) {
        self.node.set_parent(Rc::new(node));
    }

    pub fn get(&mut self) -> &mut Container {
        self
    }

    pub fn add_child(&mut self, node: Box<dyn VisualNode>) {
        self.node_tree.push(node)
    } 

    pub fn new() ->  Container {
        let node_tree : Vec<Box<dyn VisualNode>> = vec![];
        let children = vec![];
        let x:f32 = 0.0;
        let y:f32 = 0.0;
        let opacity:f32 = 0.0;
        let parent = Weak::new();
        let node = Node { x, y, opacity, parent, children };
        Container { node_tree, node }
    }
  /*  
    pub fn create(&mut self) {
        let tb1 = ToggleButtonPrivate::new (10.0, 10.0, 0.0, None);
        self.node_tree.push(Box::new(tb1));
        let mut tb2 = ButtonPrivate::new (10.0, 50.0, 0.0, None);
        self.node_tree.push(Box::new(tb2));

//        let json = serde_json::to_string(&self.node_tree).unwrap();
//        web_sys::console::log_1(&json.into());

    }
*/
    pub fn get_tree(&mut self) -> &mut [Box<dyn VisualNode>] {
        self.node_tree.as_mut_slice()
    }
}
