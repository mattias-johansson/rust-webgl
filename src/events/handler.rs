use crate::events::mouse::*;

pub struct Handler {
    pub event: Mouse
}

impl Handler {
    pub fn new() -> Handler { 
        let event = Mouse::new(0,0, MouseEvent::None);
        Handler { event }
    }

    pub fn set_event(&mut self, event: Mouse) {
        self.event = event; 
    }
}