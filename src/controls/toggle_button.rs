
use crate::rnd::texture_unit::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::draw::*;
use crate::events::mouse::*;

pub struct ToggleButton {
    x: f32,
    y: f32,
    opacity: f32,
    pressed: bool,
}

impl ToggleButton {
    pub fn new(x: f32, y: f32, opacity: f32) -> ToggleButton { 
        let pressed = false;
        ToggleButton { x, y, opacity, pressed } 
    }
}

impl Draw for ToggleButton {
    fn draw(&self, context: &WebGlRenderingContext, program: &WebGlProgram) {
        if self.pressed {
            render(&context, &program, 107.0, 36.0, self.x + 0.0, self.y + 0.0, TextureUnit::ToggelBackground);
            render(&context, &program, 30.0, 30.0, self.x + 75.0, self.y + 3.0, TextureUnit::ToggleActive);
        } else {
            render(&context, &program, 107.0, 36.0, self.x + 0.0, self.y + 0.0, TextureUnit::ToggelBackground);
            render(&context, &program, 30.0, 30.0, self.x + 3.0, self.y + 3.0, TextureUnit::Toggle);
        }
    }

    fn event(&mut self, event: &Mouse) {
        self.pressed = !self.pressed;
    }

    fn position(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + 107.0, self.y + 36.0)
    }
}
