use js_sys::{Function};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Listeners {
    objects:  Rc<RefCell<HashMap<String, HashMap<String, Vec<Function>>>>>
}

impl Listeners {
    pub fn new() -> Listeners {
        let objects : Rc<RefCell<HashMap<String, HashMap<String, Vec<Function>>>>> = Rc::new(RefCell::new(HashMap::new()));
        Listeners { objects }
    }

    pub fn get_listners_ref(&self) -> Rc<RefCell<HashMap<String, HashMap<String, Vec<Function>>>>> {
        Rc::clone(&self.objects)
    }

    pub fn add_listener(&mut self, sender: String, signal: String, callback: Function) {
        web_sys::console::log_1(&"1".into());
        let mut objects = self.objects.borrow_mut();
        web_sys::console::log_1(&"2".into());
        let sender_listers = objects.get_mut(&sender);
        web_sys::console::log_1(&"3".into());
        match sender_listers {
            Some(sender_listers) => {
                web_sys::console::log_1(&"some".into());
                let signal_listener = sender_listers.get_mut(&signal);
                match signal_listener {
                    Some(signal_listener) => {
                        web_sys::console::log_1(&"some".into());
                        signal_listener.push(callback);
                    },
                    None => {
                        web_sys::console::log_1(&"none".into());
                        let mut functions = vec![];
                        functions.push(callback);
                        sender_listers.insert(signal, functions);
                    }
                }
            },
            None => {
                web_sys::console::log_1(&"none".into());
                let mut sender_listers = HashMap::new();
                let mut functions = vec![];
                functions.push(callback);
                sender_listers.insert(signal, functions);
                objects.insert(sender, sender_listers);
            }
        }
    }
}