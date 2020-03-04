use crate::animation::ease::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Animator {
    pub running: bool,
    pub pressed: bool,
    pub easing: Ease,
    pub start_time: f32,
    pub duration: f32,
}

impl Animator {
    pub fn new(running: bool, easing: Ease, start_time: f32, duration: f32) -> Animator {
        let pressed = true;
        Animator { running, pressed, easing, start_time, duration }
    } 

    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    pub fn set_start_time(&mut self, start_time: f32) {
        self.start_time = start_time;
        self.pressed = false;
    }

    pub fn set_pressed(&mut self) {
        self.pressed = true;
    }
}