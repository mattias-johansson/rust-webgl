
extern crate serde;

use wasm_bindgen::prelude::*;
use web_sys::{Worker};
    
use serde::*;
use uuid::Uuid;

pub struct Messaging {
    pub worker: Worker
}


#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MessageType {
    ObjectCreated(String),
    ValueUpdated(Uuid, String, String),
    AddNode(String),
    RemoveNode(String),
    SetRoot(String)
}

impl Messaging {
    pub fn send_message(&self, message: MessageType) -> Result<(), JsValue> {
        web_sys::console::log_1(&"send_message".into());
        let json = serde_json::to_string(&message);
        self.worker.post_message(&json.unwrap().into())?;
        Ok(())
    }
}