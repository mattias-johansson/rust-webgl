
use uuid::Uuid;

#[derive(PartialEq, Clone, Copy)]
pub enum Event {
    None,
    Mouse(Mouse),
    Message(Message),
    Scroll(Scroll)
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum MouseEvent {
    None,
    Up,
    Down,
    Move
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Mouse {
    pub x: u16,
    pub y: u16,
    pub event: MouseEvent,
}

impl Mouse {
    pub fn new(x: u16, y: u16, event: MouseEvent ) -> Mouse { Mouse {x, y, event} }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Message {
    AnimationStarted(Uuid),
    AnimationEnded(Uuid),
}

#[derive(PartialEq, Clone, Copy)]
pub enum Scroll {
    ImmediateValue(f32),
    Value(f32),
}
