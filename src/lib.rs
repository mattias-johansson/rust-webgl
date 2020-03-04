extern crate wasm_bindgen;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};

#[macro_use]
extern crate erased_serde;
use self::page::*;
use crate::controls::toggle_button::*;
use crate::events::handler::*;
use crate::events::mouse::*;
use crate::rnd::draw::init_textures;

mod animation;
mod controls;
mod events;
mod page;
mod rnd;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {
    page: Page,
    gl: Rc<WebGlRenderingContext>,
    program: WebGlProgram,
    events: Rc<RefCell<Handler>>,
    //    renderer: WebRenderer,
}

#[wasm_bindgen]
impl Application {
    /// Create a new Application
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
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
        )
        .unwrap();

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
        )
        .unwrap();

        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

        gl.use_program(Some(&program));

        let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        let events = Handler::new();
        let events = Rc::new(RefCell::new(events));
        let mut page = Page::new();
        page.set_child(Box::new(ToggleButtonPrivate::new(10.0, 10.0, 0.0, None)));
/*
        let tb1 = 
        self.node_tree.push(Box::new(tb1));
        let mut tb2 = ButtonPrivate::new (10.0, 50.0, 0.0, None);
        self.node_tree.push(Box::new(tb2));
*/
        Application {
            page,
            gl,
            program,
            events,
        }
    }

    /// Start our application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&mut self) -> Result<(), JsValue> {
        let gl = &self.gl;
        init_textures(Rc::clone(gl));
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        attach_mouse_down_handler(&canvas, Rc::clone(&self.events));
        attach_mouse_up_handler(&canvas, Rc::clone(&self.events));

        Ok(())
    }
    pub fn event_loop(&mut self, dt: f32) {
        //        web_sys::console::log_1(&"render".into());
        //        let js: JsValue = dt.into();
        //        web_sys::console::log_1(&js);

        //////////////////////////////////////
        // First handle all events
        //
        // Currently we only support one 
        // Mouse event per frame
        /////////////////////////////////////
        let x = self.events.borrow().event.x;
        let y = self.events.borrow().event.y;
        
        if x != 0 && y != 0 {
//            for node in self.page.get_child()() {
            if let Some(node) = self.page.get_child().as_mut() {
                let xy = node.position();
                if xy.0 < x as f32 && xy.2 > x as f32 && xy.1 < y as f32 && xy.3 > y as f32 {
                    web_sys::console::log_1(&"sending event".into());
                    node.event(&self.events.borrow().event);
                }
            }
        }
        // Mark the event as handled by setting a "default" MouseEvent
        let mouse_event = Mouse::new(0, 0, MouseEvent::None);
        self.events.borrow_mut().set_event(mouse_event);

        //Draw all nodes
        //for node in self.page.get_child()() {
            
        if let Some(node) = self.page.get_child().as_mut() {
//          web_sys::console::log_1(&"drawing node".into());
            node.draw(&self.gl, &self.program, dt);
        }
        
    }
}

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
    program: &WebGlProgram,
) -> Option<WebGlUniformLocation> {
    gl.get_uniform_location(&program, uniform_name)
}

fn attach_mouse_down_handler(
    canvas: &web_sys::HtmlCanvasElement,
    handler: Rc<RefCell<Handler>>,
) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x() as u16;
        let y = event.client_y() as u16;
        let mouse_event = Mouse::new(x, y, MouseEvent::Down);
        handler.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
fn attach_mouse_up_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,
) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x() as u16;
        let y = event.client_y() as u16;
        let mouse_event = Mouse::new(x, y, MouseEvent::Up);
        events.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
