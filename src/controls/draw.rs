use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;

extern crate erased_serde;

pub trait Draw {
    fn draw(&self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32);

    fn event(&mut self, event: &Mouse);

    fn position(&self) -> (f32, f32, f32, f32);
    
}