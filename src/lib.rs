extern crate wasm_bindgen;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::application::context::*;
use crate::controls::button::*;
use crate::controls::container::*;
use crate::controls::page::*;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::Node;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::controls::toggle_button::*;
use crate::events::handler::*;
use crate::render::draw::init_textures;
use crate::render::gl_context::*;
use crate::web::events::*;

mod application;
mod animation;
mod controls;
mod events;
mod render;
mod web;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {
    page: Page,
    gl: Rc<WebGlRenderingContext>,
    program: WebGlProgram,
    events: Rc<RefCell<Handler>>,
    context: Context,
}

#[wasm_bindgen]
impl Application {
    /// Create a new Application
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
            
        let webgl_context = get_webgl_context();
        let program = create_webgl_program(&webgl_context);
        setup_redering_context(&webgl_context, &program);
        let gl = Rc::new(webgl_context);

        let events = Handler::new();
        let events = Rc::new(RefCell::new(events));
        let mut context = Context::new();
        let mut page = Page::new(&mut context);
        
        Application {
            page,
            gl,
            program,
            events,
            context
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

    /**
     * The event loop called from JavaScript
     */


    pub fn event_loop(&mut self, dt: f32) {

        let root_node = &mut self.page;
//        draw_tree(&self.gl, &self.program, Rc::clone(&self.events), root_node, dt);
        //        web_sys::console::log_1(&"render".into());
        //        let js: JsValue = dt.into();
        //        web_sys::console::log_1(&js);

        //////////////////////////////////////
        // First handle all events
        //
        // Currently we only support one 
        // Mouse event per frame
        /////////////////////////////////////
     /*   let x = self.events.borrow().event.x;
        let y = self.events.borrow().event.y;
        
        if x != 0 && y != 0 {
            let root_node = self.page.get_node();
            //for node in self.page.get_child()() {
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

        */
        
    }

}

/*
pub fn get_children(gl: &WebGlRenderingContext, 
                    program: &WebGlProgram, 
                    events: Rc<RefCell<Handler>>, 
                    visual_node: &mut VisualNode, 
                    dt: f32) {
    let node : &Node = visual_node.get_node();
    let vector: &[Rc<dyn VisualNode>] = node.get_children(); 
    for mut child in vector {
        let count = Rc::strong_count(&child);

        web_sys::console::log_1(&count.to_string().into());
        get_children(gl, program, Rc::clone(&events), Rc::get_mut(&mut child).unwrap(), dt);
    }
    //Later all send_events should be done before all draw 
    send_event(events, visual_node);
    draw(gl, program, visual_node, dt);
}

pub fn send_event(events: Rc<RefCell<Handler>>,  node: &mut VisualNode) {
    let x = events.borrow().event.x;
    let y = events.borrow().event.y;
    let xy = node.position();
    if xy.0 < x as f32 && xy.2 > x as f32 && xy.1 < y as f32 && xy.3 > y as f32 {
        web_sys::console::log_1(&"sending event".into());
        node.event(&events.borrow().event);
    }
}

pub fn draw(gl: &WebGlRenderingContext, program: &WebGlProgram, node: &mut VisualNode, dt: f32) {
    node.draw(&gl, &program, dt);
}
*/
