use crate::animation::ease::*;
use uuid::Uuid;


#[derive(PartialEq, Clone, Copy)]
pub enum Attribute {
    X,
    Y,
    OPACITY,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AnimationState {
    Stopped,
    Started,
    Ended,
    Playing,
    Ending
}

#[derive(PartialEq, Clone, Copy)]
pub struct Animation {
    pub uuid: Uuid,
    pub easing: Ease,
    pub start_time: f32,
    pub duration: f32,
    pub start_value: f32,
    pub end_value: f32,
    pub target_node: Uuid,
    pub target_attribute: Attribute,
    pub state: AnimationState,
}

impl Animation {

    pub fn new(target_node: Uuid, target_attribute: Attribute) -> Animation {
        let uuid = Uuid::new_v4();
        let easing= Ease::Lin;
        let start_time = 0.0; 
        let duration = 0.0;
        let start_value = 0.0;
        let end_value = 0.0;
        let state = AnimationState::Stopped;
        Animation { uuid, easing, start_time, duration, start_value, end_value, target_node, target_attribute, state }
    } 

    pub fn play(&mut self) {
        self.state = AnimationState::Started;
    }
    
    pub fn create(easing: Ease, start_time: f32, duration: f32, start_value: f32, end_value: f32, target_node: Uuid, target_attribute: Attribute) -> Animation {
        let state = AnimationState::Stopped;
        let uuid = Uuid::new_v4();
        Animation { uuid, easing, start_time, duration, start_value, end_value, target_node, target_attribute, state }
    } 

    pub fn running(&self, time: f32) -> bool {
        self.start_time + self.duration > time
    }

    pub fn set_start_time(&mut self, start_time: f32) {
        self.start_time = start_time;
    }

    pub fn get_animated_value(&self,  time: f32) -> f32 {
        let time = (time - self.start_time) / self.duration;
        let ease = self.easing.map((time) as f32);
        let mut x = 0.0;
        if self.end_value > self.start_value {
            let start = self.start_value;
            let end = self.end_value;
            x = ease * (end - start);
        } else {
            let start = self.start_value;
            let end = self.end_value;
            web_sys::console::log_1(&start.into());
            x = start + (ease * (end - start));
        }

        web_sys::console::log_1(&"animation value".into());
        web_sys::console::log_1(&x.into());

        x
    } 
}