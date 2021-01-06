extern crate serde;
mod controls;
mod globals;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};
use js_sys::{Function};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use self::controls::container::*;
use self::controls::button::*;
use self::globals::messaging::*;
use self::globals::listeners::*;
use serde::*;
use uuid::Uuid;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {
    listeners: Listeners
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {   
        web_sys::console::log_1(&"new application".into());
        let listeners = Listeners::new();
        add_on_message_handler(Rc::clone(&listeners.objects));
        Application { listeners }
    }

    pub fn set_root(object : Container) -> Result<(), JsValue> {
        let json = serde_json::to_string(&object).unwrap();
        let message = MessageType::SetRoot(json);
        send_message(message);
        Ok(())
    }

    pub fn add_listener(&mut self, sender: &Button, signal:String, callback: Function ) -> Result<(), JsValue> {
        self.listeners.add_listener(sender.get_uuid(), signal, callback);
        Ok(())
    }

}


