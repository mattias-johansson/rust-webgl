use web_sys::{WebGlProgram, WebGlRenderingContext};
use wasm_bindgen::JsCast;

use crate::render::shaders::compile_shader;
use crate::render::shaders::link_program;

pub fn get_webgl_context() -> WebGlRenderingContext {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let gl = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl
}

pub fn setup_redering_context(gl: &WebGlRenderingContext, program: &WebGlProgram) { 
    
    gl.use_program(Some(&program));
    
    let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    
}

pub fn create_webgl_program(gl: &WebGlRenderingContext) -> WebGlProgram {
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        precision mediump float;
        attribute vec4 vertexData;
        varying vec2 texCoords;
        
        uniform float transparency;
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 perspective;
    
        void main() {
            gl_Position = perspective * view * model * vec4(vertexData.xy, 1.0, 1.0);
            texCoords = vertexData.zw;
        }
    "#,
    )
    .unwrap();
    
    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;
        varying vec2 texCoords;
        uniform float transparency;
        uniform sampler2D texture;
    
        void main() {
            gl_FragColor = texture2D( texture, texCoords ); 
            gl_FragColor.rgb *= gl_FragColor.a;
            gl_FragColor.w = transparency * gl_FragColor.a;
        }
        "#,
    )
    .unwrap();
    
    let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

    program
}

pub fn create_webgl_program_color(gl: &WebGlRenderingContext) -> WebGlProgram {
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        precision mediump float;
        attribute vec4 vertexData;
        varying vec2 texCoords;
        
        uniform float transparency;
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 perspective;
    
        void main() {
            gl_Position = perspective * view * model * vec4(vertexData.xy, 1.0, 1.0);
            texCoords = vertexData.zw;
        }
    "#,
    )
    .unwrap();
    
    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;
        uniform float transparency;
        uniform vec3 color;
    
        void main() {
            gl_FragColor = vec4(color, 1.0);
            gl_FragColor.rgb *= gl_FragColor.a;
            gl_FragColor.w = transparency * gl_FragColor.a;
        }
        "#,
    )
    .unwrap();
    
    let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

    program
}

