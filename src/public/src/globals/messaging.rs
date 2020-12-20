
extern crate serde;

//use std::cell::RefCell;
//use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};
//use web_sys::{ErrorEvent, Event, Worker, MessageEvent};

use serde::*;


#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MessageType {
    ObjectCreated(String),
    ValueUpdated(String),
    AddNode(String),
    RemoveNode(String),
    SetRoot(String)
}

pub fn send_message(message: MessageType) -> Result<(), JsValue> {
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    let json = serde_json::to_string(&message);
    global.post_message(&json.unwrap().into())?;
    Ok(())
}
/*
pub fn send<T>(r#type: MessageType, object: &T) -> Result<(), JsValue> where T : Serialize {
    let data = serde_json::to_string(&object).unwrap();
    let message = Message { r#type, data };
    send_message(message)
}
*/