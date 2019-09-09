use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::events::mouse::*;

pub trait Draw {
    fn draw(&self, context: &WebGlRenderingContext, program: &WebGlProgram);

    fn event(&mut self, event: &Mouse);
}