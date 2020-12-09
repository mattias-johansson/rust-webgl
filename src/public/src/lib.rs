//use std::cell::RefCell;
//use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};
//use web_sys::{ErrorEvent, Event, Worker, MessageEvent};

mod controls;
use self::controls::container::*;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {

}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {            
        Application {}
    }

    pub fn add(object : &Container) -> Result<(), JsValue> {
        let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
        let json = serde_json::to_string(&object);
        global.post_message(&json.unwrap().into())?;
        Ok(())
    }
}