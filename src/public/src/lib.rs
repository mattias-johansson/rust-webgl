mod controls;
mod globals;

use uimsg::Slider;
use uimsg::send_message;
use wasm_bindgen::prelude::*;
use js_sys::{Function};

use self::globals::messaging::*;
use self::globals::listeners::*;

use uimsg::MessageType;

use uimsg::Button;
use uimsg::Container;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {
    listeners: Listeners
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {     
        #[cfg(debug_assertions)]  
        console_error_panic_hook::set_once();  

        web_sys::console::log_1(&"new application".into());
        let listeners = Listeners::new();
        add_on_message_handler(listeners.get_listners_ref());
        Application { listeners }
    }

    pub fn set_root(object : &Container) -> Result<(), JsValue> {
        let json = serde_json::to_string(&object).unwrap();
        let message = MessageType::SetRoot(json);
        send_message(message);
        Ok(())
    }

    pub fn add_listener(&mut self, sender: &Button, signal:String, callback: Function ) -> Result<(), JsValue> {
        self.listeners.add_listener(sender.get_uuid(), signal, callback);
        Ok(())
    }

    pub fn add_slider_listener(&mut self, sender: &Slider, signal:String, callback: Function ) -> Result<(), JsValue> {
        self.listeners.add_listener(sender.get_uuid(), signal, callback);
        Ok(())
    }

}


