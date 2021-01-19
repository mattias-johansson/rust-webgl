extern crate wasm_bindgen;
use uuid::Uuid;
use crate::controls::node::Node;
use crate::events::application_events::ApplicationEvents;
use crate::web::events::attach_touch_move_handler;
use crate::web::events::attach_touch_end_handler;
use crate::web::events::attach_touch_start_handler;
use crate::web::events::attach_mouse_move_handler;
use crate::application::core_app::CoreApp;

use crate::events::mouse::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::application::context::*;
use crate::controls::page::*;
use crate::controls::visual_node::VisualNode;

use web_sys::{WebGlProgram, WebGlRenderingContext, Worker};

use crate::controls::container::*;
use crate::controls::button::*;

use crate::events::handler::*;
use crate::render::draw::*;
use crate::render::gl_context::*;
use crate::web::events::*;
use crate::globals::messaging::*;

mod application;
mod animation;
mod controls;
mod events;
mod render;
mod web;
mod globals;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct Application {
    page: Page,
    gl: Rc<WebGlRenderingContext>,
    program: WebGlProgram,
    program_color: WebGlProgram,
    events: Rc<RefCell<Handler>>,
    application_events: Rc<RefCell<ApplicationEvents>>,
    context: Context,
    core_app: CoreApp,
}

#[wasm_bindgen]
impl Application {

    /// Create a new Application
    #[wasm_bindgen(constructor)]
    pub fn new(worker: &str) -> Application {            
        web_sys::console::log_1(&"Application".into());

        let worker = Worker::new(worker).unwrap();
        let _ = worker.post_message(&"startup".into());

        let application_events = ApplicationEvents::new();
        let application_events = Rc::new(RefCell::new(application_events));
        let _ = add_on_message_handler(&worker, application_events.clone());

        let webgl_context = get_webgl_context();
        let program = create_webgl_program(&webgl_context);
        let program_color = create_webgl_program_color(&webgl_context);
        
        setup_redering_context(&webgl_context, &program);
        let gl = Rc::new(webgl_context);

        let events = Handler::new();
        let events = Rc::new(RefCell::new(events));
        let mut context = Context::new(worker);
        let page = Page::new(&mut context);
        context.root = Some(page.node);
        
        let core_app = CoreApp { visual_nodes : vec![] , names : HashMap::default()};

        Application {
            page,
            gl,
            program,
            program_color,
            events,
            application_events,
            context,
            core_app,
        }
    }


    /// Start our application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&mut self) -> Result<(), JsValue> {
        web_sys::console::log_1(&"start".into());
//        create(&mut self.context, &mut self.core_app);
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let _ = attach_mouse_down_handler(&canvas, Rc::clone(&self.events));
        let _ = attach_mouse_up_handler(&canvas, Rc::clone(&self.events));
        let _ = attach_mouse_move_handler(&canvas, Rc::clone(&self.events));
        let _ = attach_touch_start_handler(&canvas, Rc::clone(&self.events));
        let _ = attach_touch_end_handler(&canvas, Rc::clone(&self.events));
        let _ = attach_touch_move_handler(&canvas, Rc::clone(&self.events));

        Ok(())
    }


    /**
     * The event loop called from JavaScript
     */
    pub fn event_loop(&mut self, dt: f32) {
        // Handle animation events & clear events when done
        let events = &self.context.events.clone();
        send_events_from_context(&mut self.context, events, &mut self.core_app.visual_nodes);  //Animation events
        self.context.events = vec![];

        handle_application_events(&mut self.context, &mut self.core_app, Rc::clone(&self.application_events));

        // Handle Mouse and Touch events (Currently only one at per frame)
        new_send_events(&mut self.context, Rc::clone(&self.events), &mut self.core_app.visual_nodes);  // Touch events
        
        let callbacks = self.context.cb.clone();
        callbacks.trigger_callbacks(&mut self.context, &mut self.core_app);
        self.context.cb.clear_triggered();

        update_animations(dt, &mut self.context);
        update_target_attributes(dt, &mut self.context);
        draw_scene(&mut self.context, Rc::clone(&self.gl), &self.program, &self.program_color);
        //panic!("one loop");
    }
}

pub fn handle_application_events(context: &mut Context, core_app: &mut CoreApp, app_events: Rc<RefCell<ApplicationEvents>>) {
    let mut events = app_events.borrow_mut();
    if &events.events.len() > &0 {
    web_sys::console::log_1(&"has events".into());
        let string = &events.events.pop().unwrap();
        web_sys::console::log_1(&string.into());
        let result = serde_json::from_str(string);
        let message : MessageType = result.unwrap();
        match message {
            MessageType::SetRoot(data) => {
                let result = serde_json::from_str(&data);                
                let object : ContainerPrivate = result.unwrap();
                let container = Container::from_private(context, object);
                context.add_child_to(context.root.unwrap(), container.get_node_uuid());
                core_app.visual_nodes.push(Box::new(container) as Box<dyn VisualNode>);          
            },
            MessageType::ObjectCreated(parent, data) => {
                web_sys::console::log_1(&"ObjectCreated".into());
                let result = serde_json::from_str(&data);                
                let object : Button = result.unwrap();
                let button = ButtonPrivate::from_public(context, object);
                let container = core_app.get_visual_node(parent).unwrap();
                context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);          
            },
            _ => ()
        }
        
    }

} 

pub fn send_events_from_context(context: &mut Context, events: &Vec<Event>, this_frame: &mut Vec<Box<dyn VisualNode>>) {
    for event in events {
        for vn in 0..this_frame.len() {
            web_sys::console::debug_1(&"sending animation event:".into());
            this_frame.get_mut(vn).unwrap().event_handler(context, &event);
        }
    }
}

pub fn new_send_events(context: &mut Context, events: Rc<RefCell<Handler>>, this_frame: &mut Vec<Box<dyn VisualNode>>) {
    let event = events.borrow().event;
    if event == Event::None {
        return
    }
    let uuid = context.root.unwrap();
    let node = context.get_node_unmut(uuid);
    let node = *node.unwrap();

    travers_tree(context, node, events, this_frame);

}

pub fn travers_tree(cx: &mut Context, parent: Node, events: Rc<RefCell<Handler>>, this_frame: &mut Vec<Box<dyn VisualNode>>) {
    let thing = cx.node_relations.clone();
    match thing.get(&parent.uuid) {
        Some(children) => {
            for child in children.iter().rev() {
                let node = cx.get_node_unmut(*child);
                let node = *node.unwrap();
                let node_owner = node.owner;
                let optional_visual_node = get_visual_node(this_frame, node_owner);
                match optional_visual_node {
                    Some(mut visual_node) => {
                        let x = node.x + parent.x + node.translate_x + parent.translate_x;
                        let y = node.y + parent.y + node.translate_y + parent.translate_y;
                        let x1 = x + node.width;
                        let y1 = y + node.height;
                        send_event(Rc::clone(&events), cx, (x, y, x1, y1), &mut visual_node);
                    },
                    _ => ()
                }
                travers_tree(cx, node, Rc::clone(&events), this_frame);
            }
        }
        None => (),
    }
}

pub fn get_visual_node(frame: &mut Vec<Box<dyn VisualNode>>, uuid: Uuid) -> Option<&mut Box<dyn VisualNode>> {
    let visual_nodes: &mut Vec<Box<dyn VisualNode>> = frame.as_mut(); 
    for visual_node in visual_nodes {
        if uuid == visual_node.get_uuid() {
            return Some(visual_node);
        }
    }
    None
}

pub fn send_events(context: &mut Context, events: Rc<RefCell<Handler>>, this_frame: &mut Vec<Box<dyn VisualNode>>) {

    let event = events.borrow().event;
    if event == Event::None {
        return
    }

    for vn in (0..this_frame.len()).rev() {
        let mut visual_node = this_frame.get_mut(vn).unwrap();
        let root_node_uuid = visual_node.get_node_uuid();
        let node = context.get_node_unmut(root_node_uuid).unwrap();
        let position = node.position();
        web_sys::console::debug_2(&"sending to visual_node ".into(), &node.owner.to_string().into());
        
        send_event(Rc::clone(&events), context, position, &mut visual_node)
    }

    let event = Event::None;
    events.borrow_mut().set_event(event);   
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
                if xy.0 < x as f32 && xy.2 > x as f32 && xy.1 < y as f32 && xy.3 > y as f32 {
                    web_sys::console::debug_4(&"x".into(), &x.to_string().into(), &"y".into(), &y.to_string().into());
                    handled = visual_node.event_handler(cx, &events.borrow().event);
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

pub fn get_canvas_size() -> (u32, u32) { 
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

   (canvas.width(), canvas.height())
}

pub fn get_device_pixel_ratio() -> f64 { 
    let window = web_sys::window().unwrap();
    let window = window.dyn_into::<web_sys::Window>().unwrap();
    window.device_pixel_ratio()
}
/*
pub fn create(mut context: &mut Context, core_app: &mut CoreApp) {
    let slider = Slider::new(&mut context, 500.0, 500.0, 1.0);

    let container = ContainerBuilder::builder().x(0.0).height(703.0).width(1280.0).color((1.0,1.0,1.0)).opacity(1.0).clip(true).build(&mut context);
    //let container2 = ContainerBuilder::builder().x(20.0).y(20.0).width(400.0).height(400.0).color((0.0,1.0,1.0)).opacity(0.5).build(&mut context);
    let label = Label::new(context, 45.0, 45.0, "Test label");
    context.add_child_to(context.root.unwrap(), container.get_node_uuid());
    container.add_child(&mut context, slider.get_node_uuid());
    //container.add_child(&mut context, container2.get_node_uuid());
    
    
    //let scroll_view = ScrollView::new(&mut context, 0.0, 0.0, 0.0, 0.0, 0.5, 300.0, 300.0, (1.0,0.0,1.0), );
//    scroll_view.add_content(&mut context, image_view.get_node_uuid());
    //context.add_child_to(context.root.unwrap(), scroll_view.get_node_uuid());

    let button = ButtonPrivate::new(Uuid::new_v4(), &mut context, "Button", 150.0, 5.0, 0.5);

    container.add_child(&mut context,button.get_node_uuid());
    container.add_child(&mut context,label.get_node_uuid());

    context.cb.add_subscriber(button.this, on_button_pressed);
    context.cb.add_subscriber(slider.this, on_slider_event);

    core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
    core_app.visual_nodes.push(Box::new(slider) as Box<dyn VisualNode>);
//    core_app.visual_nodes.push(Box::new(scroll_view) as Box<dyn VisualNode>);

}

pub fn on_button_pressed(mut cx: &mut Context, core_app: &mut CoreApp, event: Event) {
    let y = cx.nodes.len() as f32 * 20.0;
    let button = ToggleButtonPrivate::new(&mut cx, 5.0, y, 1.0);
    cx.add_child_to(cx.root.unwrap(), button.get_node_uuid());
    core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
}

pub fn on_slider_event(mut cx: &mut Context, core_app: &mut CoreApp, event: Event) {
    cx.dirty = true;
    let bild = core_app.get_visual_node_from_name("Bild");
    match event {
        Event::Scroll(scroll) => {
            match scroll {
                Scroll::ImmediateValue(value) => {
                    web_sys::console::log_1(&"scroll".into());
                    web_sys::console::log_1(&value.to_string().into());
                    let node_uuid = bild.unwrap().get_node_uuid();
                    let node = cx.get_node(node_uuid);
                    node.unwrap().opacity = value;
                },
                _ => ()
            }
        },
        _ => ()
    }
}
*/