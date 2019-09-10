
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum MouseEvent {
    None,
    Up,
    Down
}

pub struct Mouse {
    pub x: u16,
    pub y: u16,
    pub event: MouseEvent,
}

impl Mouse {
    pub fn new(x: u16, y: u16, event: MouseEvent ) -> Mouse { Mouse {x, y, event} }
}
