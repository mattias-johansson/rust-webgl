use wasm_bindgen::prelude::*;
use web_sys::{Worker};

use uimsg::MessageType;

pub struct Messaging {
    pub worker: Worker
}

impl Messaging {
    pub fn send_message(&self, message: MessageType) -> Result<(), JsValue> {
        web_sys::console::log_1(&"send_message".into());
        let json = serde_json::to_string(&message);
        self.worker.post_message(&json.unwrap().into())?;
        Ok(())
    }
}