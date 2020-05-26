
use uuid::Uuid;

#[derive(PartialEq, Clone, Copy)]
pub enum Event {
    Mouse(Mouse),
    Message(Message),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum MouseEvent {
    None,
    Up,
    Down
}

#[derive(PartialEq, Clone, Copy)]
pub struct Mouse {
    pub x: u16,
    pub y: u16,
    pub event: MouseEvent,
}

impl Mouse {
    pub fn new(x: u16, y: u16, event: MouseEvent ) -> Mouse { Mouse {x, y, event} }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Message {
    AnimationStarted(Uuid),
    AnimationEnded(Uuid),
}
