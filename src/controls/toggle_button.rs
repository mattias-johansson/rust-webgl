
use std::rc::Rc;
use crate::rnd::texture_unit::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::draw::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animator::*;
use crate::animation::ease::*;
use serde::{Serialize, Deserialize};
extern crate erased_serde;

pub struct ToggleButton {
    node: Node,
    value: bool,
    animator: Animator,
    event_handler: Option<Box<Fn(&MouseEvent)>>,
}

impl ToggleButton {
    pub fn new(x: f32, y: f32, opacity: f32, parent: Option<Rc<Node>>) -> ToggleButton { 
        let node = Node { x, y, opacity, parent };
        let value = false;
        let animator = Animator::new(false, Ease::OutCubic, 0.0, 250.0);
        let event_handler = Option::None;
        ToggleButton { node, value, animator, event_handler } 
    }
}

impl Draw for ToggleButton {
    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {
        if(self.animator.pressed) {
            self.animator.setStartTime(time);
        }
        if self.value {        
            render(&context, &program, 107.0, 36.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::ToggelBackground);
            let end = self.node.x + 74.0;
            if self.animator.running {
                let time = (time - self.animator.start_time) / self.animator.duration;

                let x = self.animator.easing.map((time) as f32);
                let start = self.node.x + 3.0;
//                let js: JsValue = x.into();
//                web_sys::console::log_1(&js);

                let x = x * (end - start);
                render(&context, &program, 30.0, 30.0, start + x, self.node.y + 3.0, TextureUnit::ToggleActive);
                if x == 1.0 {
                    self.animator.setRunning(false);
                }
            } else {
                render(&context, &program, 30.0, 30.0, end, self.node.y + 3.0, TextureUnit::ToggleActive);
            }
        } else {
            render(&context, &program, 107.0, 36.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::ToggelBackground);
            let end = self.node.x + 3.0;
            if self.animator.running {
                let time = (time - self.animator.start_time) / self.animator.duration;

                let x = self.animator.easing.map((time) as f32);
                let start = self.node.x + 74.0;
//                let js: JsValue = x.into();
//                web_sys::console::log_1(&js);

                let x = x * (end - start);

                render(&context, &program, 30.0, 30.0, start + x, self.node.y + 3.0, TextureUnit::Toggle);
            } else {
                render(&context, &program, 30.0, 30.0, end, self.node.y + 3.0, TextureUnit::Toggle);
            }
        }
    }

    fn event(&mut self, event: &Mouse) {
        if event.event == MouseEvent::Up {
            self.animator = Animator::new(true, Ease::OutCubic, 0.0, 250.0);
            self.value = !self.value;
        }
        match self.event_handler {
            Some(ref handler) => handler(&event.event),
            None => (),
        }
    }

    fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 107.0, self.node.y + 36.0)
    }
}
