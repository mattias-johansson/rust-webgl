use web_sys::WebGlRenderingContext as GL;

#[derive(Clone, Copy)]
pub enum TextureUnit {
    Button = 0,
    Toggle = 1,
    ToggleActive = 2,
    ToggelBackground = 3,
}

impl TextureUnit {
    /// gl.TEXTURE1, gl.TEXTURE2 ... etc. Useful for `gl.active_texture`
    #[allow(non_snake_case)]
    pub fn TEXTURE_N(&self) -> u32 {
        match self {
            TextureUnit::Button => GL::TEXTURE0,
            TextureUnit::Toggle => GL::TEXTURE1,
            TextureUnit::ToggleActive => GL::TEXTURE2,
            TextureUnit::ToggelBackground => GL::TEXTURE3,
        }
    }

    /// 0, 1, 2, ... etc. Useful for `gl.uniform1i` calls
    pub fn texture_unit(&self) -> i32 {
        *self as i32
    }
}
