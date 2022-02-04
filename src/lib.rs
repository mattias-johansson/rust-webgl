use crate::application::core_app::CoreApp;
use crate::controls::image_view::ImageViewPrivate;
use crate::controls::label::LabelPrivate;
use crate::controls::list_view::ListViewPrivate;
use crate::controls::node::Node;
use crate::controls::scroll_view::ScrollViewPrivate;
use crate::controls::slider::SliderPrivate;
use crate::controls::toggle_button::ToggleButtonPrivate;
use crate::events::application_events::ApplicationEvents;
use crate::web::events::attach_mouse_move_handler;
use crate::web::events::attach_touch_end_handler;
use crate::web::events::attach_touch_move_handler;
use crate::web::events::attach_touch_start_handler;
use uuid::Uuid;

use crate::application::context::*;
use crate::controls::page::*;
use crate::controls::visual_node::VisualNode;
use crate::events::mouse::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{WebGlProgram, WebGlRenderingContext, Worker};

use serde_json::{Map, Value};

use crate::controls::button::*;
use crate::controls::container::*;

use crate::events::handler::*;
use crate::render::draw::*;
use crate::render::gl_context::*;
use crate::web::events::*;

use uimsg::MessageType;

mod animation;
mod application;
mod controls;
mod events;
mod globals;
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
    application_events: Rc<RefCell<ApplicationEvents>>,
    context: Context,
    core_app: CoreApp,
    node_relations: HashMap<Uuid, Vec<Uuid>>,
    collection: Vec<Node>,
}

#[wasm_bindgen]
impl Application {
    /// Create a new Application
    #[wasm_bindgen(constructor)]
    pub fn new(worker: &str) -> Application {
        #[cfg(debug_assertions)]
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

        let core_app = CoreApp {
            visual_nodes: vec![],
            names: HashMap::default(),
        };
        let node_relations = HashMap::default();
        let collection = Vec::new();

        Application {
            page,
            gl,
            program,
            program_color,
            events,
            application_events,
            context,
            core_app,
            node_relations,
            collection,
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
        let _ = attach_window_resize_handler(&web_sys::window().unwrap(), Rc::clone(&self.events));
        resize(&self.context);
        Ok(())
    }

    /**
     * The event loop called from JavaScript
     */
    pub fn event_loop(&mut self, dt: f32) {
        // Handle animation events & clear events when done
        let events = &self.context.events.clone();
        send_events_from_context(&mut self.context, events, &mut self.core_app.visual_nodes); //Animation events
        self.context.events = vec![];

        handle_application_events(
            &mut self.context,
            &mut self.core_app,
            Rc::clone(&self.application_events),
        );

        // Handle Mouse and Touch events (Currently only one at per frame)
        new_send_mouse_events(
            &mut self.context,
            Rc::clone(&self.events),
            &mut self.core_app.visual_nodes,
            &mut self.node_relations,
        ); // Touch events
        update_animations(dt, &mut self.context);
        update_target_attributes(dt, &mut self.context);

        draw_scene(
            &mut self.context,
            Rc::clone(&self.gl),
            &self.program,
            &self.program_color,
            &mut self.collection,
        );
        self.collection.clear();
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
pub fn handle_application_events(
    context: &mut Context,
    core_app: &mut CoreApp,
    app_events: Rc<RefCell<ApplicationEvents>>,
) {
    let mut events = app_events.borrow_mut();
    while &events.events.len() > &0 {
        //    web_sys::console::log_1(&"has events".into());
        let string = &events.events.pop().unwrap();
        //        web_sys::console::log_1(&string.into());
        let result = serde_json::from_str(string);
        let message: MessageType = result.unwrap();
        match message {
            MessageType::SetRoot(data) => {
                let result = serde_json::from_str(&data);
                let object: Container = result.unwrap();
                let container = ContainerPrivate::from_public(context, object);
                context.add_child_to(context.root.unwrap(), container.get_node_uuid());
                core_app
                    .visual_nodes
                    .push(Box::new(container) as Box<dyn VisualNode>);
            }
            MessageType::ObjectCreated(parent, object_type, data) => {
                web_sys::console::log_1(&"ObjectCreated".into());
                if object_type == "button" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = ButtonPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "togglebutton" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = ToggleButtonPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "imageview" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = ImageViewPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "label" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = LabelPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "slider" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let button = SliderPrivate::from_public(context, object);
                    let container = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(container.get_node_uuid(), button.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(button) as Box<dyn VisualNode>);
                } else if object_type == "container" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let child = ContainerPrivate::from_public(context, object);
                    let parent = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(parent.get_node_uuid(), child.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(child) as Box<dyn VisualNode>);
                } else if object_type == "scrollview" {
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let child = ScrollViewPrivate::from_public(context, object);
                    let parent = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(parent.get_node_uuid(), child.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(child) as Box<dyn VisualNode>);
                } else if object_type == "listview" {
                    web_sys::console::debug_1(&"Got ListView message".into());
                    let result = serde_json::from_str(&data);
                    let object = result.unwrap();
                    let child = ListViewPrivate::from_public(context, object);
                    let parent = core_app.get_visual_node(parent).unwrap();
                    context.add_child_to(parent.get_node_uuid(), child.get_node_uuid());
                    core_app
                        .visual_nodes
                        .push(Box::new(child) as Box<dyn VisualNode>);
                }
            }
            MessageType::SetupScrollView(scroll_view, data) => {
                web_sys::console::debug_1(&"SetupScrollView event".into());
                let result = serde_json::from_str(&data);
                let object = result.unwrap();
                let child = ContainerPrivate::from_public(context, object);
                let child_uuid = child.get_node_uuid();
                core_app
                    .visual_nodes
                    .push(Box::new(child) as Box<dyn VisualNode>);
                let option_visual_node = core_app.get_visual_node(scroll_view);
                if let Some(visual_node) = option_visual_node {
                    if let Some(scroll_view) =
                        visual_node.as_any().downcast_mut::<ScrollViewPrivate>()
                    {
                        web_sys::console::debug_1(&"scroll_view.setup_scroll".into());
                        scroll_view.setup_scroll(context, child_uuid);
                    }
                }
            }
            MessageType::SetupListView(list_view, data) => {
                web_sys::console::debug_1(&"SetupListlView event".into());
                web_sys::console::debug_1(&data.clone().into());
                let result: Vec<Map<String, Value>> = serde_json::from_str(&data).unwrap();
                let mut i = 0.0;
                for value in result {
                    let item: Map<String, Value> = value;
                    for (key, value) in item {
                        web_sys::console::debug_2(&key.into(), &value.as_str().unwrap().into());
                        let child = LabelPrivate::new(
                            Uuid::new_v4(),
                            context,
                            0.0,
                            i,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            &value.as_str().unwrap(),
                        );
                        i = i + 10.0;
                        let child_uuid = child.get_node_uuid();
                        core_app
                            .visual_nodes
                            .push(Box::new(child) as Box<dyn VisualNode>);
                        let option_visual_node = core_app.get_visual_node(list_view);
                        if let Some(visual_node) = option_visual_node {
                            web_sys::console::debug_1(&"Some(visual_node)".into());
                            if let Some(list_view) =
                                visual_node.as_any().downcast_mut::<ListViewPrivate>()
                            {
                                web_sys::console::debug_1(&"list_view.setup_list".into());
                                context.add_child_to(list_view.scroll_uuid, child_uuid);
                            }
                        }
                    }
                }
                let option_visual_node = core_app.get_visual_node(list_view);
                if let Some(visual_node) = option_visual_node {
                    if let Some(list_view) = visual_node.as_any().downcast_mut::<ListViewPrivate>()
                    {
                        list_view.setup_list(context);
                    }
                }
            }
            MessageType::ObjectRemoved(parent, child) => {
                web_sys::console::debug_1(&"Remove object".into());

                let parent = core_app.get_visual_node_un_mut(parent).unwrap();
                let child = core_app.get_visual_node_un_mut(child).unwrap();
                context.remove_child_from(parent.get_node_uuid(), child.get_node_uuid());
            }
            MessageType::ValueUpdated(this, object_type, json) => {
                if object_type == "label" {
                    let result = serde_json::from_str(&json);
                    let object = result.unwrap();
                    let new_label = LabelPrivate::from_public(context, object);
                    let old_label = core_app
                        .get_visual_node(this)
                        .unwrap()
                        .as_any()
                        .downcast_mut::<LabelPrivate>()
                        .unwrap();
                    old_label.text(context, &new_label.text);
                } else if object_type == "opacity" {
                    let node_uuid = core_app.get_visual_node(this).unwrap().get_node_uuid();
                    let node = context.get_node(node_uuid).unwrap();
                    node.set_opacity(json.parse::<f32>().unwrap());
                } else if object_type == "x" {
                    let node_uuid = core_app.get_visual_node(this).unwrap().get_node_uuid();
                    let node = context.get_node(node_uuid).unwrap();
                    node.set_x(json.parse::<f32>().unwrap());
                } else if object_type == "y" {
                    let node_uuid = core_app.get_visual_node(this).unwrap().get_node_uuid();
                    let node = context.get_node(node_uuid).unwrap();
                    node.set_y(json.parse::<f32>().unwrap());
                } else if object_type == "width" {
                    let node_uuid = core_app.get_visual_node(this).unwrap().get_node_uuid();
                    let node = context.get_node(node_uuid).unwrap();
                    node.set_width(json.parse::<f32>().unwrap());
                } else if object_type == "height" {
                    let node_uuid = core_app.get_visual_node(this).unwrap().get_node_uuid();
                    let node = context.get_node(node_uuid).unwrap();
                    node.set_height(json.parse::<f32>().unwrap());
                } else if object_type == "translate_x" {
                    let node_uuid = core_app.get_visual_node(this).unwrap().get_node_uuid();
                    let node = context.get_node(node_uuid).unwrap();
                    node.set_translate_x(json.parse::<f32>().unwrap());
                } else if object_type == "translate_y" {
                    let node_uuid = core_app.get_visual_node(this).unwrap().get_node_uuid();
                    let node = context.get_node(node_uuid).unwrap();
                    node.set_translate_y(json.parse::<f32>().unwrap());
                }
            }
            _ => (),
        }
    }
}

pub fn send_events_from_context(
    context: &mut Context,
    events: &Vec<Event>,
    this_frame: &mut Vec<Box<dyn VisualNode>>,
) {
    for event in events {
        for vn in 0..this_frame.len() {
            web_sys::console::debug_1(&"sending animation event:".into());
            this_frame
                .get_mut(vn)
                .unwrap()
                .event_handler(context, &event, Uuid::new_v4());
        }
    }
}

pub fn new_send_mouse_events(
    context: &mut Context,
    events: Rc<RefCell<Handler>>,
    this_frame: &mut Vec<Box<dyn VisualNode>>,
    node_relations: &mut HashMap<Uuid, Vec<Uuid>>,
) {
    {
        let event = events.borrow().event;
        if event == Event::None {
            return;
        }
    }
    let uuid = context.root.unwrap();
    let node = context.get_node_unmut(uuid);
    let node = *node.unwrap();
    node_relations.clone_from(&context.node_relations);
    {
        let event = &events.borrow().event;
        let mut matched_nodes: Vec<Uuid> = vec![];
        match event {
            Event::Mouse(event) => {
                if event.event != MouseEvent::None {
                    //web_sys::console::debug_4(&"event_x".into(), &event.x.to_string().into(), &"event_y".into(), &event.y.to_string().into());
                    matched_nodes = travers_tree(context, node, *event, this_frame, node_relations);
                    matched_nodes.dedup();
                    //web_sys::console::debug_2(&"matched_nodes:".into(), &matched_nodes.len().to_string().into());
                    if event.event == MouseEvent::Down {
                        let target = matched_nodes.get(matched_nodes.len() - 1).unwrap();
                        context.touch_target = Some(*target);
                        web_sys::console::debug_2(&"Setting touch_target:".into(), &target.to_string().into());
                    } 
                    if event.event == MouseEvent::Move && context.touch_target.is_some() {
                        if context.touch_target.unwrap() != *matched_nodes.get(matched_nodes.len() - 1).unwrap() {
                            matched_nodes.push(context.touch_target.unwrap());
                            web_sys::console::debug_2(&"sending touch_target:".into(), &context.touch_target.unwrap().to_string().into());
                        }
                    }
                    if event.event == MouseEvent::Up {
                        if context.touch_target.unwrap() != *matched_nodes.get(matched_nodes.len() - 1).unwrap() {
                            matched_nodes.push(context.touch_target.unwrap());
                            web_sys::console::debug_1(&"adding touch_target".into());
                        }
                        context.touch_target = None

                    }
                }
            }
            Event::Window(_resize) => {
                web_sys::console::debug_1(&"Window resized".into());
                resize(&context);
            }
            _ => {
                web_sys::console::debug_1(&"FAILED".into());
            }
        }
        if matched_nodes.len() > 0 {
            let target = matched_nodes.get(matched_nodes.len() - 1).unwrap();
            let target = *target;
            
            web_sys::console::debug_2(&"Target is: ".into(), &target.to_string().into());
            for node in matched_nodes {
                //                web_sys::console::debug_1(&"node".into());
                let opt_visual_node = get_visual_node(this_frame, node);
                match opt_visual_node {
                    Some(visual_node) => {
                        //                        web_sys::console::debug_1(&"node".into());
                        //                        web_sys::console::debug_1(&visual_node.get_uuid().to_string().into());
                        visual_node.event_handler(context, &events.borrow().event, target);
                    }
                    _ => (),
                }
            }
        }
    }
    let new_event = Event::None;
    events.borrow_mut().set_event(new_event);
}

pub fn travers_tree(
    cx: &mut Context,
    parent: Node,
    event: Mouse,
    this_frame: &mut Vec<Box<dyn VisualNode>>,
    node_relations: &HashMap<Uuid, Vec<Uuid>>,
) -> Vec<Uuid> {
    let mut matched: Vec<Uuid> = vec![];
    match node_relations.get(&parent.uuid) {
        Some(children) => {
            //            web_sys::console::debug_2(&"children: ".into(), &children.len().to_string().into());
            for child in children.iter().rev() {
                let node = cx.get_node_unmut(*child);
                let node = *node.unwrap();
                let node_owner = node.owner;
                let optional_visual_node = get_visual_node(this_frame, node_owner);
                match optional_visual_node {
                    Some(visual_node) => {
                        //                        web_sys::console::debug_2(&"YES visual for node_owner: ".into(), &node_owner.to_string().into());
                        let x = node.x() + parent.x() + node.translate_x() + parent.translate_x();
                        let y = node.y() + parent.y() + node.translate_y() + parent.translate_y();
                        let x1 = x + node.width();
                        let y1 = y + node.height();

                        let event_x = event.x;
                        let event_y = event.y;
                        let previous_event_x = event_x - event.movement_x;
                        let previous_event_y = event_y - event.movement_y;
                        //                        web_sys::console::debug_5(&visual_node.get_uuid().to_string().into(), &"y1".into(), &y1.to_string().into(), &"y".into(), &y.to_string().into());
                        if x1 >= event_x as f32
                            && x <= event_x as f32
                            && y1 >= event_y as f32
                            && y <= event_y as f32
                        {
                            //send_event(*child, Rc::clone(&events), cx, (x, y, x1, y1), &mut visual_node);
                            //                            web_sys::console::debug_2(&"target: ".into(), &node_owner.to_string().into());
                            //                            web_sys::console::debug_5(&visual_node.get_uuid().to_string().into(), &"event_x: ".into(), &event_x.to_string().into(), &"event_y: ".into(), &event_y.to_string().into());
                            //                            web_sys::console::debug_5(&visual_node.get_uuid().to_string().into(), &"previous_event_x: ".into(), &previous_event_x.to_string().into(), &"previous_event_y: ".into(), &previous_event_y.to_string().into());
                            matched.push(visual_node.get_uuid());
                            if !(x1 >= previous_event_x as f32
                                && x <= previous_event_x as f32
                                && y1 >= previous_event_y as f32
                                && y <= previous_event_y as f32)
                            {
                                //                                web_sys::console::debug_2(&"Mouse IN for: ".into(), &node_owner.to_string().into());
                                let event =
                                    Event::Mouse(Mouse::new(event_x, event_y, MouseEvent::In));
                                visual_node.event_handler(cx, &event, visual_node.get_uuid());
                                //                              web_sys::console::debug_5(&visual_node.get_uuid().to_string().into(), &"event_x: ".into(), &event_x.to_string().into(), &"event_y: ".into(), &event_y.to_string().into());
                                //                              web_sys::console::debug_5(&visual_node.get_uuid().to_string().into(), &"previous_event_x: ".into(), &previous_event_x.to_string().into(), &"previous_event_y: ".into(), &previous_event_y.to_string().into());
                            }
                        } else {
                            if x1 >= previous_event_x as f32
                                && x <= previous_event_x as f32
                                && y1 >= previous_event_y as f32
                                && y <= previous_event_y as f32
                            {
                                //                                web_sys::console::debug_2(&"Mouse OUT for: ".into(), &node_owner.to_string().into());
                                let event =
                                    Event::Mouse(Mouse::new(event_x, event_y, MouseEvent::Out));
                                visual_node.event_handler(cx, &event, visual_node.get_uuid());
                                //                              web_sys::console::debug_5(&visual_node.get_uuid().to_string().into(), &"event_x: ".into(), &event_x.to_string().into(), &"event_y: ".into(), &event_y.to_string().into());
                                //                              web_sys::console::debug_5(&visual_node.get_uuid().to_string().into(), &"previous_event_x: ".into(), &previous_event_x.to_string().into(), &"previous_event_y: ".into(), &previous_event_y.to_string().into());
                            }
                        }
                        let mut new_node = node;
                        {
                            new_node.set_x(node.x() + parent.x());
                            new_node.set_y(node.y() + parent.y());
                            new_node.set_translate_x(node.translate_x() + parent.translate_x());
                            new_node.set_translate_y(node.translate_y() + parent.translate_y());
                        }
                        let mut ret_val =
                            travers_tree(cx, new_node, event, this_frame, node_relations);
                        matched.append(&mut ret_val);
                    }
                    _ => {
                        //                        web_sys::console::debug_2(&"NO visual for node_owner: ".into(), &node_owner.to_string().into());
                    }
                }
            }
        }
        None => (),
    }
    matched
}

pub fn get_visual_node(
    frame: &mut Vec<Box<dyn VisualNode>>,
    uuid: Uuid,
) -> Option<&mut Box<dyn VisualNode>> {
    let visual_nodes: &mut Vec<Box<dyn VisualNode>> = frame.as_mut();
    for visual_node in visual_nodes {
        if uuid == visual_node.get_uuid() {
            return Some(visual_node);
        }
    }
    None
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

pub fn get_window_size() -> (u32, u32) {
    let window = web_sys::window().unwrap();
    let window = window.dyn_into::<web_sys::Window>().unwrap();
    (
        window.inner_width().unwrap().as_f64().unwrap() as u32,
        window.inner_height().unwrap().as_f64().unwrap() as u32,
    )
}

pub fn set_canvas_size(context: &Context, width: u32, height: u32) {
    web_sys::console::debug_4(
        &"Setting canvas size, width: ".into(),
        &width.to_string().into(),
        &" height: ".into(),
        &height.to_string().into(),
    );
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    let mut dirty = context.dirty.borrow_mut();
    *dirty = true;
}

pub fn resize(context: &Context) {
    let size = get_window_size();
    set_canvas_size(&context, size.0, size.1);
}
