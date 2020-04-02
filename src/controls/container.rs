
use std::rc::Rc;
use std::rc::Weak;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use crate::controls::button::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use std::any::Any;
use crate::events::mouse::*;

pub struct Container {
    node: Node,
    node_tree: Vec<Box<dyn VisualNode>>,
}

impl VisualNode for Container {

    fn get_node(&self) -> &Node {
        &self.node
    }

    fn add_child(&mut self, node: Rc<VisualNode>) {
        self.node.add_child(node);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_parent(&mut self, node: Rc<VisualNode>) {
        self.node.set_parent(node);
    }

    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {

    }

    fn event(&mut self, event: &Mouse) {

    }

    fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 107.0, self.node.y + 36.0)
    }
}

impl Container {

    fn add_parent(&mut self, node: Rc<dyn VisualNode>) {
        self.node.set_parent(node);
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
        let parent : Weak<ButtonPrivate> = Weak::new();
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
