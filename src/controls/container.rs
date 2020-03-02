
use crate::events::mouse::*;
use crate::controls::draw::Draw;
use crate::controls::button::ButtonPrivate;
use crate::controls::node::*;

pub struct Container {
    node: Node,
    node_tree: Vec<Box<dyn Draw>>,
}

impl Container {

    pub fn get(&mut self) -> &mut Container {
        self
    }

    pub fn addChild(&mut self, node: Box<dyn Draw>) {
        self.node_tree.push(node)
    } 

    pub fn new() ->  Container {
        let node_tree : Vec<Box<dyn Draw>> = vec![];
        let children = vec![];
        let x:f32 = 0.0;
        let y:f32 = 0.0;
        let opacity:f32 = 0.0;
        let parent = None;
        let node = Node { x, y, opacity, parent, children };
        Container { node_tree, node }
    }

    pub fn do_things(&mut self, event: &MouseEvent) {
        self.node_tree.push(Box::new(ButtonPrivate::new (10.0, 90.0, 0.0, None)));
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
    pub fn get_tree(&mut self) -> &mut [Box<dyn Draw>] {
        self.node_tree.as_mut_slice()
    }
}
