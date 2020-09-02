extern crate wasm_bindgen;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::application::context::*;
use crate::controls::page::*;
use crate::controls::visual_node::VisualNode;

use crate::events::mouse::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::controls::container::*;
use crate::controls::toggle_button::*;
use crate::controls::button::*;
use crate::controls::image_view::*;
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
    program_color: WebGlProgram,
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
        let program_color = create_webgl_program_color(&webgl_context);
        
        setup_redering_context(&webgl_context, &program);
        let gl = Rc::new(webgl_context);

        let events = Handler::new();
        let events = Rc::new(RefCell::new(events));
        let mut context = Context::new();
        let page = Page::new(&mut context);
        context.root = Some(page.node);
        
        let visual_nodes = Rc::new(RefCell::new(vec![]));
        let visual_nodes2 = Rc::new(RefCell::new(vec![]));

        Application {
            page,
            gl,
            program,
            program_color,
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
        draw_scene(&mut self.context, Rc::clone(&self.gl), &self.program, &self.program_color);
        while self.visual_nodes2.borrow_mut().len() > 0 {
            self.visual_nodes.borrow_mut().push(self.visual_nodes2.borrow_mut().pop().unwrap());
         }
    }

    pub fn send_events_from_context(&mut self) {
        let events =  &self.context.events.clone();
        let cx = &mut self.context; 
        let mut vn = self.visual_nodes.borrow_mut();
        for event in events {
            for i in 0..vn.len() {
//                web_sys::console::log_1(&"ev2:".into());
                    vn.get_mut(i).unwrap().event_handler(cx, &event);
            }
        }
        self.context.events = vec![];
    }

    pub fn send_events(&mut self) {
        let nodes =  &self.context.nodes.clone();

        let event = self.events.borrow().event;
        if event == Event::None {
            return
        }

        let cx = &mut self.context; 
        for node in nodes.iter().rev() {  //todo iteraton over both nodes and visual nodes.
            let mut vn = self.visual_nodes.borrow_mut();
            for i in 0..vn.len() {
                let mut vn = vn.get_mut(i).unwrap();

                if vn.get_uuid() == node.owner {
                    web_sys::console::log_2(&"sending to node ".into(), &i.to_string().into());
                    send_event(Rc::clone(&self.events), cx, node.position(), &mut vn)
                }
            }
        }   
    }
}


pub fn create(mut context: &mut application::context::Context, visual_nodes2: Rc<RefCell<Vec<Box<dyn VisualNode>>>>) {
    
    let image_view = ImageViewBuilder::builder().x(0.0).y(500.0).width(400.0).height(400.0).opacity(1.0).image("IMG_20160408_164451.jpg").build(&mut context);

    let container = ContainerBuilder::builder().x(100.0).height(200.0).width(200.0).color((1.0,1.0,0.0)).opacity(0.5).clip(true).build(&mut context);
    let container2 = ContainerBuilder::builder().x(20.0).y(20.0).width(200.0).height(200.0).color((0.0,1.0,1.0)).opacity(0.5).build(&mut context);
    context.add_child_to(context.root.unwrap(), container.get_node_uuid());
    container.add_child(&mut context, container2.get_node_uuid());
    context.add_child_to(context.root.unwrap(), image_view.get_node_uuid());

    let mut button = ButtonPrivate::new(&mut context, 150.0, 5.0, 0.5);
    container.add_child(&mut context, button.get_node_uuid());
    let v_n = Rc::clone(&visual_nodes2);
/*
    let handler = move |mut cx : &mut Context| {
        let mut visual_nodes2 = visual_nodes2.borrow_mut();
        let y = 45 + cx.visual_nodes.len() * 45; //TODO Cannot get other nodes here
        web_sys::console::log_1(&cx.visual_nodes.len().to_string().into());
        let button = ToggleButtonPrivate::new(&mut cx, 50.0, y as f32, 1.0);
        cx.add_child_to(cx.root.unwrap(), button.get_node_uuid());
        cx.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
        visual_nodes2.push(Box::new(ToggleButtonPrivate::new(&mut cx, 50.0, y as f32, 1.0)) as Box<dyn VisualNode>);
    };
*/
    let handler = move |mut cx : &mut Context| {
//        let visual_node = cx.get_visual_node(button.get_uuid()).unwrap();
//        let almost_button = ButtonPrivate::to_button_private(visual_node.as_any());
//        almost_button.unwrap().
        let &mut mut node = cx.get_node(button.get_node_uuid()).unwrap();
        node.x = node.x + 5.0;
    };

    let handler : Box<dyn FnMut(&mut Context)> = Box::new(handler) as Box<dyn FnMut(&mut Context)>;
    let mut v_n = v_n.borrow_mut();
    context.callbacks.insert(button.get_uuid(), handler);
    v_n.push(Box::new(image_view) as Box<dyn VisualNode>);
    //v_n.push(Box::new(container2) as Box<dyn VisualNode>);
    //v_n.push(Box::new(container) as Box<dyn VisualNode>);
    v_n.push(Box::new(button) as Box<dyn VisualNode>);
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
//              web_sys::console::log_1(&visual_node.get_uuid().to_string().into());
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
        let event = Event::None;
        events.borrow_mut().set_event(event);
    }
}