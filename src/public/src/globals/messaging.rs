extern crate serde;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};

use serde::*;
use uuid::Uuid;
use js_sys::{Function};
use std::collections::HashMap;




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

pub fn add_on_message_handler(objects: HashMap<Uuid, HashMap<String, Function>>) -> Result<(), JsValue> {
    
    web_sys::console::log_1(&"add_on_message_handler".into());
    web_sys::console::log_1(&objects.len().to_string().into());

    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    let handler = move |event: web_sys::MessageEvent| {
        web_sys::console::log_1(&"message".into());
          let data = event.data().as_string().unwrap();  
            let result = serde_json::from_str(&data);
            let message : MessageType = result.unwrap();
            match message {
                MessageType::ValueUpdated(uuid, signal, value) => {
                    web_sys::console::log_1(&uuid.to_string().into());
                    web_sys::console::log_1(&"value updated signal1".into());
                    web_sys::console::log_1(&objects.len().to_string().into());
                    let optional_listener = objects.get(&uuid);
                    web_sys::console::log_1(&"value updated signal2".into());
                    let listener = optional_listener.unwrap();
                    web_sys::console::log_1(&"value updated signal3".into());
                    let optional_callback = listener.get(&signal);
                    web_sys::console::log_1(&"value updated signal4".into());
                    let callback = optional_callback.unwrap();
                    web_sys::console::log_1(&"value updated signal5".into());
                    let globalic = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
                    callback.call1(&JsValue::from(globalic), &JsValue::from(&value));
                },
                _ => ()
            }
        };
    
        let handler = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    
        global.set_onmessage(Some(handler.as_ref().unchecked_ref()));
        handler.forget();
    Ok(())
}

/*
pub fn send<T>(r#type: MessageType, object: &T) -> Result<(), JsValue> where T : Serialize {
    let data = serde_json::to_string(&object).unwrap();
    let message = Message { r#type, data };
    send_message(message)
}
*/