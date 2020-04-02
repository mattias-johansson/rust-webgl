use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use crate::controls::button::*;
use std::rc::Weak;
use std::rc::Rc;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use std::any::Any;
use crate::events::mouse::*;

pub struct Page {
    node: Node,
    child: Option<Box<dyn VisualNode>>,
}

impl VisualNode for Page {

    fn get_node(&self) -> &Node {
        &self.node
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_child(&mut self, node: Rc<VisualNode>) {
        self.node.add_child(node);
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

impl Page {

    pub fn get(&mut self) -> &mut Page {
        self
    }
/*
    pub fn set_child(&mut self, node: Box<dyn VisualNode>) {
        self.child = Some(node);
    } 
*/
    pub fn new() ->  Page {
        let child = None;
        let children = vec![];
        let x:f32 = 0.0;
        let y:f32 = 0.0;
        let opacity:f32 = 0.0;
        let parent : Weak<ButtonPrivate> = Weak::new();
        let node = Node { x, y, opacity, parent, children };
        Page { node, child }
    }
/*
    pub fn get_child(&mut self) -> &mut Option<Box<dyn VisualNode>> {
        &mut self.child
    }
    */
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