use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};
use web_sys::{ErrorEvent, Event, Worker};

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

    pub fn add() -> Result<(), JsValue> {
        let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
        global.post_message(&"Test".into())?;
        Ok(())
    }
}