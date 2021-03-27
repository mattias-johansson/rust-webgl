
use crate::controls::scroll_view::ScrollViewPrivate;
use crate::controls::slider::SliderPrivate;
use crate::controls::label::LabelPrivate;
use crate::controls::image_view::ImageViewPrivate;
use crate::controls::toggle_button::ToggleButtonPrivate;
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

use uimsg::MessageType;

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
        console_error_panic_hook::set_once();  
        web_sys::console::log_1(&"Application".into());

        let worker = Worker::new(worker).unwrap();
//        let _ = worker.post_message(&"startup".into());

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
/*
pub fn object_creator<R, T>(cx: &mut Context, public:R) ->  T
    where
        R: Clone,
        T: VisualNode,
{
    ButtonPrivate::new(Uuid::new_v4(), &mut context, "Button", 150.0, 5.0, 0.5)
}
*/
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
                let object : Container = result.unwrap();
                let container = ContainerPrivate::from_public(context, object);
                context.add_child_to(context.root.unwrap(), container.get_node_uuid());
                core_app.visual_nodes.push(Box::new(container) as Box<dyn VisualNode>);          
            },
            MessageType::ObjectCreated(parent, object_type, data) => {
                web_sys::console::log_1(&"ObjectCreated".into());
                if object_type == "button" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = ButtonPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "togglebutton" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = ToggleButtonPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "imageview" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = ImageViewPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "label" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = LabelPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "slider" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = SliderPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app.visual_nodes.push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "container" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let child = ContainerPrivate::from_public(context, object);
                    let parent = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(parent.get_node_uuid(), child.get_node_uuid());
                    core_app.visual_nodes.push(Box::new(child) as Box<dyn VisualNode>);
                } else if object_type == "scrollview" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let child = ScrollViewPrivate::from_public(context, object);
                    let parent = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(parent.get_node_uuid(), child.get_node_uuid());
                    core_app.visual_nodes.push(Box::new(child) as Box<dyn VisualNode>);
                }
            },
            MessageType::SetupScrollView(scroll_view, data) => {
                web_sys::console::debug_1(&"SetupScrollView event".into());
                let result = serde_json::from_str(&data);
                let object = result.unwrap();
                let child = ContainerPrivate::from_public(context, object);
                let child_uuid = child.get_node_uuid();
                core_app.visual_nodes.push(Box::new(child) as Box<dyn VisualNode>);
                let option_visual_node = core_app.get_visual_node(scroll_view);
                if let Some(visual_node) = option_visual_node {
                    if let Some(scroll_view) = visual_node.as_any().downcast_mut::<ScrollViewPrivate>() {
                        web_sys::console::debug_1(&"scroll_view.setup_scroll".into());
                        scroll_view.setup_scroll(context, child_uuid);
                    }
                }
            },
            MessageType::ObjectRemoved(parent, child) => {
                web_sys::console::debug_1(&"Remove object".into());

                let parent = core_app.get_visual_node_un_mut(parent).unwrap();
                let child = core_app.get_visual_node_un_mut(child).unwrap();
                context.remove_child_from(parent.get_node_uuid(), child.get_node_uuid());
            }
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
                        let x = node.x() + parent.x() + node.translate_x() + parent.translate_x();
                        let y = node.y() + parent.y() + node.translate_y() + parent.translate_y();
                        let x1 = x + node.width();
                        let y1 = y + node.height();
                        send_event(Rc::clone(&events), cx, (x, y, x1, y1), &mut visual_node);
                    },
                    _ => ()
                }
                travers_tree(cx, node, Rc::clone(&events), this_frame);
            }
        }
        None => (),
    }
    let event = Event::None;
    events.borrow_mut().set_event(event);   
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
//                    web_sys::console::debug_4(&"x".into(), &x.to_string().into(), &"y".into(), &y.to_string().into());
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