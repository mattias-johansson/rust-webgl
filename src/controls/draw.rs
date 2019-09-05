use web_sys::{WebGlProgram, WebGlRenderingContext};

pub trait Draw {
    fn draw(&self, context: &WebGlRenderingContext, program: &WebGlProgram);
}