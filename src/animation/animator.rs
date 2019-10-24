use crate::animation::ease::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
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