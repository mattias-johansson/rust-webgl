use crate::rnd::texture_unit::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::draw::*;
use crate::events::mouse::*;

pub struct Button {
    x: f32,
    y: f32,
    opacity: f32,
    pressed: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, opacity: f32) -> Button { 
        let pressed = false;    
        Button { x, y, opacity, pressed } 
    }
}

impl Draw for Button {
    fn draw(&self, context: &WebGlRenderingContext, program: &WebGlProgram) {
        if self.pressed {
            render(&context, &program, 145.0, 34.0, self.x + 0.0, self.y + 0.0, TextureUnit::ButtonPressed);
        } else {
            render(&context, &program, 145.0, 34.0, self.x + 0.0, self.y + 0.0, TextureUnit::Button);    
        }
    }

    fn event(&mut self, event: &Mouse) {
        if event.event == MouseEvent::Up {
            self.pressed = false;
        } else if event.event == MouseEvent::Down {
            self.pressed = true; 
        }
    }
    
   fn position(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + 145.0, self.y + 34.0)
    }
}