extern crate wasm_bindgen;
use crate::rnd::draw::init_textures;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};
use std::f32::consts::PI;
use std::rc::Rc;
use self::rnd::texture_unit::*;
use self::rnd::lti::*;
use self::rnd::draw::*;
use js_sys::WebAssembly;
use nalgebra_glm as glm;
use std::cell::RefCell;
use crate::controls::draw::Draw;

use crate::controls::toggle_button::*;
use crate::controls::button::*;

mod rnd;
mod controls;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {
    gl: Rc<WebGlRenderingContext>,
    node_tree: Vec<Box<dyn Draw>>,
    program: WebGlProgram,
//    renderer: WebRenderer,
}

#[wasm_bindgen]
impl Application {

    /// Create a new Application
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let gl = canvas
        .get_context("webgl").unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>().unwrap();

    let gl = Rc::new(gl);

    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
      r#"
        attribute vec4 vertexData;
        varying vec2 texCoords;
        
        uniform mat4 model;
            uniform mat4 view;
        uniform mat4 perspective;

        void main() {
            gl_Position = perspective * view * model * vec4(vertexData.xy, 1.0, 1.0);
            texCoords = vertexData.zw;
        }
    "#,
    ).unwrap();

    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
        precision mediump float;
        varying vec2 texCoords;
        uniform sampler2D texture;

        void main() {
            gl_FragColor = texture2D( texture, texCoords ); 
            gl_FragColor.rgb *= gl_FragColor.a;
        }
        "#,
    ).unwrap();

    let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

    gl.use_program(Some(&program));

    let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let node_tree : Vec<Box<dyn Draw>> = vec![];

        Application { gl, node_tree, program }
    }

    /// Start our application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&mut self) -> Result<(), JsValue> {
        let gl = &self.gl;
        init_textures(Rc::clone(gl));
        let tb1 = ToggleButton::new (10.0, 10.0, 0.0);
        self.node_tree.push(Box::new(tb1));
        let tb2 = Button::new (10.0, 50.0, 0.0);
        self.node_tree.push(Box::new(tb2));

        Ok(())
    }
 
    pub fn render(&mut self) {
        for node in self.node_tree.iter() {
            node.draw(&self.gl, &self.program);
        }
}

}
/*
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    
    let mut node_tree : Vec<ToggleButton> = vec![];
    let tb1 = ToggleButton::new (10.0, 10.0, 0.0);
    node_tree.push(tb1);
    let tb2 = ToggleButton::new (30.0, 30.0, 0.0);
    node_tree.push(tb2);
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let context = Rc::new(context);

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
      r#"
        attribute vec4 vertexData;
        varying vec2 texCoords;
        
        uniform mat4 model;
            uniform mat4 view;
        uniform mat4 perspective;

        void main() {
            gl_Position = perspective * view * model * vec4(vertexData.xy, 1.0, 1.0);
            texCoords = vertexData.zw;
        }
    "#,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
        precision mediump float;
        varying vec2 texCoords;
        uniform sampler2D texture;

        void main() {
            gl_FragColor = texture2D( texture, texCoords ); 
            gl_FragColor.rgb *= gl_FragColor.a;
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_color(1.0, 1.0, 1.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

 //       render(&context, &program, 107.0, 36.0, 0.0, 0.0, TextureUnit::ToggelBackground);
       // render(&context, &program, 30.0, 30.0, 3.0, 3.0, TextureUnit::Toggle);
       // render(&context, &program, 30.0, 30.0, 3.0, 3.0, TextureUnit::ToggleActive);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
*/
pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn get_uniform_location(
    gl: &WebGlRenderingContext,
    uniform_name: &str,
    program: &WebGlProgram
) -> Option<WebGlUniformLocation> {
            gl.get_uniform_location(&program, uniform_name)
}
