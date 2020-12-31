extern crate serde;
mod controls;
mod globals;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};

use std::collections::HashMap;

use self::controls::container::*;
use self::globals::messaging::*;
use serde::*;
use uuid::Uuid;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {
    objects:  HashMap<Uuid, String>
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {   
        let objects : HashMap<Uuid, String> = HashMap::new();
        Application { objects }
    }

    pub fn set_root(object : Container) -> Result<(), JsValue> {
        let json = serde_json::to_string(&object).unwrap();
        let message = MessageType::SetRoot(json);
        send_message(message);
        Ok(())
    }
/*
    fn add_on_message_handler(
        events: Rc<RefCell<ApplicationEvents>>) -> Result<(), JsValue> {
        
        let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
        let handler = move |event: web_sys::MessageEvent| {
            let data = event.data();  
                events.borrow_mut().add_event(data.as_string().unwrap()); 
        
            };
        
            let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        
            global.set_onmessage(Some(handler.as_ref().unchecked_ref()));
            handler.forget();
        Ok(())
    }
    */
}
