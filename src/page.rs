
use std::rc::Weak;
use std::rc::Rc;

use crate::events::handler::Handler;
use crate::events::mouse::*;
use crate::controls::draw::Draw;
use crate::controls::toggle_button::*;
use crate::controls::button::*;
use crate::events::click_handler::ClickHandler;

pub struct Page {
    node_tree: Vec<Box<dyn Draw>>,
    handler: Rc<Box<Fn(&MouseEvent)>>,
}

impl Page {

    pub fn get(&mut self) -> &mut Page {
        self
    }

    pub fn new() ->  Page {
        let node_tree : Vec<Box<dyn Draw>> = vec![];
        let closure = move |event: &MouseEvent| {
            web_sys::console::log_1(&"click".into());
            self.node_tree.push(Box::new(Button::new (50.0, 90.0, 0.0, None)));
        };
        let handler = Rc::new(Box::new(closure) as Box<Fn(&MouseEvent)>);

        Page { node_tree, handler }
    }

    pub fn do_things(&mut self, event: &MouseEvent) {
        self.node_tree.push(Box::new(Button::new (10.0, 90.0, 0.0, None)));
    }
    
    pub fn create(&mut self) {
        let tb1 = ToggleButton::new (10.0, 10.0, 0.0, None);
        self.node_tree.push(Box::new(tb1));
        let mut tb2 = Button::new (10.0, 50.0, 0.0, None);


        //Some(Page::do_things)
        let weak_ref = Rc::downgrade(&self.handler);
        tb2.set_click_handler(weak_ref);
        self.node_tree.push(Box::new(tb2));

//        let json = serde_json::to_string(&self.node_tree).unwrap();
//        web_sys::console::log_1(&json.into());

    }

    pub fn get_tree(&mut self) -> &mut [Box<dyn Draw>] {
        self.node_tree.as_mut_slice()
    }
}

impl ClickHandler<MouseEvent> for Page {
    fn do_things(&mut self, event : MouseEvent) {
        self.node_tree.push(Box::new(Button::new (10.0, 90.0, 0.0, None)));
    }
}