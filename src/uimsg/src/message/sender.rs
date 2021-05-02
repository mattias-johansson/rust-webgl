use web_sys::{DedicatedWorkerGlobalScope};
use wasm_bindgen::JsCast;
use crate::MessageType;

pub fn send_message(message: MessageType) {
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    let json = serde_json::to_string(&message).unwrap();
    
    match global.post_message(&json.into()) {
        Ok(()) =>  () ,
        Err(js_value) =>  { web_sys::console::error_2(&"failed to send message".into(), &js_value); }
    }
}