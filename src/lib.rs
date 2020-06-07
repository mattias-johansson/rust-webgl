extern crate wasm_bindgen;
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::application::context::*;
use crate::controls::button::*;
use crate::controls::container::*;
use crate::controls::page::*;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::Node;
use crate::events::mouse::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::controls::toggle_button::*;
use crate::events::handler::*;
use crate::render::draw::*;
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
    visual_nodes: Rc<RefCell<Vec<Box<dyn VisualNode>>>>,
    visual_nodes2: Rc<RefCell<Vec<Box<dyn VisualNode>>>>
}

#[wasm_bindgen]
impl Application {
    /// Create a new Application
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
            
        web_sys::console::log_1(&"Application".into());
        let webgl_context = get_webgl_context();
        let program = create_webgl_program(&webgl_context);
        setup_redering_context(&webgl_context, &program);
        let gl = Rc::new(webgl_context);

        let events = Handler::new();
        let events = Rc::new(RefCell::new(events));
        let mut context = Context::new();
        let page = Page::new(&mut context);
        
        let visual_nodes = Rc::new(RefCell::new(vec![]));
        let visual_nodes2 = Rc::new(RefCell::new(vec![]));

        Application {
            page,
            gl,
            program,
            events,
            context,
            visual_nodes,
            visual_nodes2
        }
    }

    /// Start our application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&mut self) -> Result<(), JsValue> {
        web_sys::console::log_1(&"start".into());
        create(&mut self.context, Rc::clone(&self.visual_nodes2));

        if self.visual_nodes2.borrow_mut().len() > 0 {
           self.visual_nodes.borrow_mut().push(self.visual_nodes2.borrow_mut().pop().unwrap());
        }

        self.visual_nodes2 =  Rc::new(RefCell::new(vec![]));

        let gl = &self.gl;
        init_textures(Rc::clone(gl));
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let _ = attach_mouse_down_handler(&canvas, Rc::clone(&self.events));
        let _ = attach_mouse_up_handler(&canvas, Rc::clone(&self.events));

        Ok(())
    }

    /**
     * The event loop called from JavaScript
     */
    pub fn event_loop(&mut self, dt: f32) {
        self.send_events_from_context();
        self.send_events();
        update_animations(dt, &mut self.context);
        update_target_attributes(dt, &mut self.context);
        draw_scene(&self.gl, &self.program, self.context.nodes.as_slice());
    }

    pub fn send_events_from_context(&mut self) {
        let events =  &self.context.events.clone();
        let cx = &mut self.context; 
        let mut vn = self.visual_nodes.borrow_mut();
        for event in events {
            for i in 0..vn.len() {
                web_sys::console::log_1(&"ev2:".into());
                    vn.get_mut(i).unwrap().event_handler(cx, &event);
            }

        }
        self.context.events = vec![];
        while self.visual_nodes2.borrow_mut().len() > 0 {
           self.visual_nodes.borrow_mut().push(self.visual_nodes2.borrow_mut().pop().unwrap());
        }
        self.visual_nodes2 =  Rc::new(RefCell::new(vec![]));
    }

    pub fn send_events(&mut self) {
        let nodes =  &self.context.nodes.clone();
        let cx = &mut self.context; 
        for node in nodes {
            let mut vn = self.visual_nodes.borrow_mut();
            web_sys::console::log_1(&vn.len().to_string().into());
            for i in 0..vn.len() {
                let mut vn = vn.get_mut(i).unwrap();

                if vn.get_uuid() == node.parent {
                    send_event(Rc::clone(&self.events), cx, node.position(), &mut vn)
                }
            }
        }
        while self.visual_nodes2.borrow_mut().len() > 0 {
           self.visual_nodes.borrow_mut().push(self.visual_nodes2.borrow_mut().pop().unwrap());
        }
        self.visual_nodes2 =  Rc::new(RefCell::new(vec![]));
    }

}


pub fn create(mut context: &mut application::context::Context, visual_nodes: Rc<RefCell<Vec<Box<dyn VisualNode>>>>) {
  
    let mut toggle_button = ToggleButtonPrivate::new(&mut context, 5.0, 5.0, 0.5);
    let v_n = Rc::clone(&visual_nodes);
    let handler = move |mut cx : &mut Context| {

        let mut visual_nodes = visual_nodes.borrow_mut();
        let y = 45 + visual_nodes.len() * 45;
        let button = ToggleButtonPrivate::new(&mut cx, 5.0, y as f32, 0.5);
        visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
    };

    let handler : Box<dyn FnMut(&mut Context)> = Box::new(handler) as Box<dyn FnMut(&mut Context)>;
    let mut v_n = v_n.borrow_mut();
    toggle_button.closure = Some(handler);
    v_n.push(Box::new(toggle_button) as Box<dyn VisualNode>);
}

pub fn send_event(events: Rc<RefCell<Handler>>, cx: &mut Context, xy: (f32, f32, f32, f32), visual_node: &mut Box<dyn VisualNode>) {
    let mut handled = false;
    {
        let event = &events.borrow().event;
        match event {
            Event::Mouse(event) => {
                if event.event != MouseEvent::None {
                let x = event.x;
                let y = event.y;

              web_sys::console::log_1(&visual_node.get_uuid().to_string().into());
                if xy.0 < x as f32 && xy.2 > x as f32 && xy.1 < y as f32 && xy.3 > y as f32 {
//                    web_sys::console::log_1(&"sending event".into());
                    handled = visual_node.event_handler(cx, &events.borrow().event);
                      web_sys::console::log_1(&"sent event".into());
                }
            }
        },
        _ => ()
        }
    }
    if handled {
        web_sys::console::log_1(&"handled".into());
        let mouse_event = Event::Mouse(Mouse::new(0, 0, MouseEvent::None));
        events.borrow_mut().set_event(mouse_event);
    }
}