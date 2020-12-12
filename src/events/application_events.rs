use crate::ContainerPrivate;

pub struct ApplicationEvents {
    pub events: Vec<String>,
}

impl ApplicationEvents {

    pub fn new() -> ApplicationEvents {
        ApplicationEvents { events: vec![] }
    }

    pub fn add_event(&mut self, data: String) {
        self.events.push(data);
    }
}