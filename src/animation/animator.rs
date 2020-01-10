use crate::animation::ease::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Animator {
    running: bool,
    pub easing: Ease,
    pub start_time: f32,
    pub duration: f32,
}

impl Animator {
    pub fn new(running: bool, easing: Ease, start_time: f32, duration: f32) -> Animator {
        Animator { running, easing, start_time, duration }
    } 

    pub fn setRunning(&mut self, running: bool) {
        self.running = running;
    }

    pub fn setStartTime(&mut self, start_time: f32) {
        self.start_time = start_time;
    }
}