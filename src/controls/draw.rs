use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;

pub trait Draw {
    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32);

    fn event(&mut self, event: &Mouse);

    fn position(&self) -> (f32, f32, f32, f32);
    
}