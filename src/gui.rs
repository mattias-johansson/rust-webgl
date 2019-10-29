
use crate::events::mouse::*;
use crate::controls::draw::Draw;
use crate::controls::toggle_button::*;
use crate::controls::button::*;
use crate::events::click_handler::ClickHandler;

pub struct Gui {
    node_tree: Vec<Box<dyn Draw>>,
}

impl Gui {

    pub fn new() -> Gui {
        let node_tree : Vec<Box<dyn Draw>> = vec![];
        Gui { node_tree }
    }

    pub fn do_things(event: &MouseEvent) {
//        self.node_tree.push(Box::new(Button::new (10.0, 90.0, 0.0)));
    }
    
    pub fn create(self) {
        let handler: fn(event: &MouseEvent) = Gui::do_things();
        let tb1 = ToggleButton::new (10.0, 10.0, 0.0);
        self.node_tree.push(Box::new(tb1));
        let mut tb2 = Button::new (10.0, 50.0, 0.0);
/*        let closure = move |event: &MouseEvent| {
            web_sys::console::log_1(&"click".into());
            nodes.push(Box::new(Button::new (10.0, 90.0, 0.0)));
        };
        let handler = Some(Box::new(closure) as Box<Fn(&MouseEvent)>);
*/
        tb2.set_click_handler(Some(handler));
        self.node_tree.push(Box::new(tb2));

//        let json = serde_json::to_string(&self.node_tree).unwrap();
//        web_sys::console::log_1(&json.into());

    }

    pub fn get_tree(self) -> Vec<Box<dyn Draw>> {
        self.node_tree
    }
}