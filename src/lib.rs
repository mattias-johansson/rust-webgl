extern crate wasm_bindgen;
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
/*
    pub fn get_owner(&self, node: Uuid) -> &Box<dyn VisualNode> {
        let visual_node = self.visual_nodes.first(); //TODO
        let visual_node = visual_node.unwrap();
        visual_node
    }
    */
    /**
     * The event loop called from JavaScript
     */


    pub fn event_loop(&mut self, dt: f32) {
        self.send_events_from_context();
        self.send_events();
        update_animations(dt, &mut self.context);
        update_target_attributes(dt, &mut self.context);
        draw_scene(&self.gl, &self.program, self.context.nodes.as_slice());
 
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
            let mut visual_nodes = self.visual_nodes.borrow_mut(); //TODO
            let optional_visual_node = visual_nodes.first_mut();
            let mut visual_node = optional_visual_node.unwrap();
            send_event(Rc::clone(&self.events), cx, node.position(), &mut visual_node)
        }
        while self.visual_nodes2.borrow_mut().len() > 0 {
           self.visual_nodes.borrow_mut().push(self.visual_nodes2.borrow_mut().pop().unwrap());
        }
        self.visual_nodes2 =  Rc::new(RefCell::new(vec![]));
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
*/

pub fn create(mut context: &mut application::context::Context, visual_nodes: Rc<RefCell<Vec<Box<dyn VisualNode>>>>) {
  

    let mut toggle_button = ToggleButtonPrivate::new(&mut context, 5.0, 5.0, 0.5);
    let v_n = Rc::clone(&visual_nodes);
    let handler = move |mut cx : &mut Context| {
        let button = ButtonPrivate::new(&mut cx, 5.0, 45.0, 0.5);
        web_sys::console::log_1(&"Click:".into());
        let mut visual_nodes = visual_nodes.borrow_mut(); //TODO
        web_sys::console::log_1(&"Click:".into());
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

/*              web_sys::console::log_1(&"EVENT:".into());
                web_sys::console::log_1(&x.to_string().into());
                web_sys::console::log_1(&y.to_string().into());
                web_sys::console::log_1(&"POSITION:".into());
                web_sys::console::log_1(&xy.0.to_string().into());
                web_sys::console::log_1(&xy.1.to_string().into());
                web_sys::console::log_1(&xy.2.to_string().into());
                web_sys::console::log_1(&xy.3.to_string().into());
*/
                if xy.0 < x as f32 && xy.2 > x as f32 && xy.1 < y as f32 && xy.3 > y as f32 {
//                    web_sys::console::log_1(&"sending event".into());
                    handled = visual_node.event_handler(cx, &events.borrow().event);
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
/*
pub fn draw(gl: &WebGlRenderingContext, program: &WebGlProgram, node: &mut VisualNode, dt: f32) {
    node.draw(&gl, &program, dt);
}
*/
