
use crate::rnd::texture_unit::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::draw::*;
use crate::events::mouse::*;
use crate::animation::animator::*;
use crate::animation::ease::*;
use wasm_bindgen::JsValue;

pub struct ToggleButton {
    x: f32,
    y: f32,
    opacity: f32,
    pressed: bool,
    animator: Option<Animator>,
}

impl ToggleButton {
    pub fn new(x: f32, y: f32, opacity: f32) -> ToggleButton { 
        let pressed = false;
        let animator = Option::None;
        ToggleButton { x, y, opacity, pressed, animator } 
    }
}

impl Draw for ToggleButton {
    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {
        if self.pressed {
            if self.animator.is_none() {
                self.animator = Option::Some(Animator::new(Ease::Lin, time, time + 3000.0));
                
            }
            render(&context, &program, 107.0, 36.0, self.x + 0.0, self.y + 0.0, TextureUnit::ToggelBackground);
            let default = self.x + 75.0;
            if self.animator.is_some() {
                let time = (time - self.animator.unwrap().start_time) / 3000.0;

                let x = self.animator.map_or(default, |a| a.easing.map(time) as f32);

                let js: JsValue = x.into();
                web_sys::console::log_1(&js);
                let x = x * (self.x + 75.0);
                render(&context, &program, 30.0, 30.0, x, self.y + 3.0, TextureUnit::ToggleActive);
            } else {
                render(&context, &program, 30.0, 30.0, default, self.y + 3.0, TextureUnit::ToggleActive);
            }
        } else {
            render(&context, &program, 107.0, 36.0, self.x + 0.0, self.y + 0.0, TextureUnit::ToggelBackground);
            render(&context, &program, 30.0, 30.0, self.x + 3.0, self.y + 3.0, TextureUnit::Toggle);
        }
    }

    fn event(&mut self, event: &Mouse) {
        if event.event == MouseEvent::Up {
            self.pressed = !self.pressed;
        }
    }

    fn position(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + 107.0, self.y + 36.0)
    }
}
