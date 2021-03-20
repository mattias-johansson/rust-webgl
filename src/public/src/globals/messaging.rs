
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope};

use js_sys::{Function};
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use uimsg::MessageType;

pub fn add_on_message_handler(objects: Rc<RefCell<HashMap<String, HashMap<String, Vec<Function>>>>>) {
    
    web_sys::console::log_1(&"add_on_message_handler".into());
//    web_sys::console::log_1(&objects.len().to_string().into());

    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    let handler = move |event: web_sys::MessageEvent| {
        let objects = objects.borrow();
        web_sys::console::log_1(&"message".into());
          let data = event.data().as_string().unwrap();  
            let result = serde_json::from_str(&data);
            let message : MessageType = result.unwrap();
            match message {
                MessageType::ValueUpdated(uuid, signal, value) => {
                    web_sys::console::log_1(&"value updated signal".into());
                    let optional_listener = objects.get(&uuid.to_string());
                    match optional_listener {
                        Some(listener) => {
                            let optional_callbacks = listener.get(&signal);
                            match optional_callbacks {
                                Some(callbacks) => {
                                    for callback in callbacks {
                                        let globalic = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
                                        match callback.call1(&JsValue::from(globalic), &JsValue::from(&value)) {
                                            Ok(_js_value) => (),
                                            Err(js_value) => { web_sys::console::error_2(&"Failed to call listener".into(), &js_value); }
                                        }
                                    }
                                },
                                _ => ()
                            }
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        };
    
        let handler = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    
        global.set_onmessage(Some(handler.as_ref().unchecked_ref()));
        handler.forget();
}

/*
pub fn send<T>(r#type: MessageType, object: &T) -> Result<(), JsValue> where T : Serialize {
    let data = serde_json::to_string(&object).unwrap();
    let message = Message { r#type, data };
    send_message(message)
}
*/