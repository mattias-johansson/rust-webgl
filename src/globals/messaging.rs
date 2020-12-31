
extern crate serde;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};

use serde::*;
use uuid::Uuid;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MessageType {
    ObjectCreated(String),
    ValueUpdated(Uuid, String, String),
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