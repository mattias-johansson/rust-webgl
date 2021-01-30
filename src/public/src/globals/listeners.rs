use js_sys::{Function};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Listeners {
    pub objects:  Rc<RefCell<HashMap<String, HashMap<String, Vec<Function>>>>>
}

impl Listeners {
    pub fn new() -> Listeners {
        let objects : Rc<RefCell<HashMap<String, HashMap<String, Vec<Function>>>>> = Rc::new(RefCell::new(HashMap::new()));
        Listeners { objects }
    }

    pub fn add_listener(&mut self, sender: String, signal: String, callback: Function) {
        let mut objects = self.objects.borrow_mut();
        let sender_listers = objects.get_mut(&sender);
        match sender_listers {
            Some(sender_listers) => {
                let signal_listener = sender_listers.get_mut(&signal);
                match signal_listener {
                    Some(signal_listener) => {
                        signal_listener.push(callback);
                    },
                    None => {
                        let mut functions = vec![];
                        functions.push(callback);
                        sender_listers.insert(signal, functions);
                    }
                }
            },
            None => {
                let mut sender_listers = HashMap::new();
                let mut functions = vec![];
                functions.push(callback);
                sender_listers.insert(signal, functions);
                objects.insert(sender, sender_listers);
            }
        }
    }
}