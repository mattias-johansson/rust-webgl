pub struct ApplicationEvents {
    pub events: Vec<String>,
}

impl ApplicationEvents {

    pub fn new() -> ApplicationEvents {
        ApplicationEvents { events: vec![] }
    }

    pub fn add_event(&mut self, data: String) {
        
        web_sys::console::log_1(&"ON MESSAGE: added event".into());
        web_sys::console::log_1(&data.to_string().into());
        
        self.events.push(data);
    }
}