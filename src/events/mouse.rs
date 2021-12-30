
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
    Move,
    In,
    Out
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub movement_x: i32,
    pub movement_y: i32,
    pub event: MouseEvent,
}

impl Mouse {
    
    pub fn new(x: i32, y: i32, event: MouseEvent ) -> Mouse {
        let movement_x = 0;
        let movement_y = 0;
        Mouse {x, y, movement_x, movement_y, event} 
    }

    pub fn new_2(x: i32, y: i32, movement_x: i32, movement_y: i32, event: MouseEvent ) -> Mouse { Mouse {x, y, movement_x, movement_y, event} }
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
