use crate::events::mouse::*;

pub struct Handler {
    pub event: Event
}

impl Handler {
    pub fn new() -> Handler { 
        let event = Event::None;
        Handler { event }
    }

    pub fn set_event(&mut self, event: Event) {
        self.event = event; 
    }
}
