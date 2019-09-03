
use crate::rnd::texture_unit::*;
use crate::rnd::lti::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};

pub struct ToggleButton {
    x: f32,
    y: f32,
    opacity: f32,
}

impl ToggleButton {
    pub fn new(x: f32, y: f32, opacity: f32) -> ToggleButton { ToggleButton { x, y, opacity } }

    pub fn draw(context: WebGlRenderingContext, program: WebGlProgram) {
            render(&context, &program, 107.0, 36.0, 0.0, 0.0, TextureUnit::ToggelBackground);
            render(&context, &program, 30.0, 30.0, 3.0, 3.0, TextureUnit::Toggle);
            render(&context, &program, 30.0, 30.0, 3.0, 3.0, TextureUnit::ToggleActive);
    }
    
}
