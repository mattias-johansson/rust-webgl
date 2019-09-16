use crate::animation::ease::*;

#[derive(PartialEq, Clone, Copy)]
pub struct Animator {
    pub easing: Ease,
    pub start_time: f32,
    pub duration: f32,
}

impl Animator {
    pub fn new(easing: Ease, start_time: f32, duration: f32) -> Animator {
        Animator { easing, start_time, duration }
    } 
}