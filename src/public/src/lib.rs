extern crate serde;
mod controls;
mod globals;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};

use self::controls::container::*;
use self::globals::messaging::*;
use serde::*;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {            
        Application {}
    }

    pub fn set_root(object : &Container) -> Result<(), JsValue> {
        let json = serde_json::to_string(&object).unwrap();
        let message = MessageType::SetRoot(json);
        send_message(message);
        Ok(())
    }
    
}
/*

*/
