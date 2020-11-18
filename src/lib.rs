extern crate wasm_bindgen;
use crate::controls::slider::Slider;
use crate::web::events::attach_touch_move_handler;
use crate::web::events::attach_touch_end_handler;
use crate::web::events::attach_touch_start_handler;
use crate::web::events::attach_mouse_move_handler;
use crate::application::core_app::CoreApp;
use crate::controls::node::Node;

use crate::events::mouse::*;
use uuid::Uuid;
use std::collections::HashMap;
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
use crate::controls::container::*;
use crate::controls::toggle_button::*;
use crate::controls::scroll_view::*;
use crate::controls::button::*;
use crate::controls::image_view::*;
use crate::controls::label::*;
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
    animation_event_listerner: HashMap<Uuid, fn(&mut Context)>,
    core_app: CoreApp,
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

        let animation_event_listerner = HashMap::default();
        
        let core_app = CoreApp { visual_nodes : vec![] , names : HashMap::default()};

        Application {
            page,
            gl,
            program,
            program_color,
            events,
            context,
            animation_event_listerner,
            core_app,
        }
    }

    /// Start our application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&mut self) -> Result<(), JsValue> {
        web_sys::console::log_1(&"start".into());
        create(&mut self.context, &mut self.core_app);
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
        //Handle animation events & clear events when done
        let events =  &self.context.events.clone();
        send_events_from_context(&mut self.context, events, &mut self.core_app.visual_nodes); //Animation events
        self.context.events = vec![];


        //Handle Mouse and Touch events (Currently only one at per frame)
        send_events(&mut self.context, Rc::clone(&self.events), &mut self.core_app.visual_nodes);              // Touch events
        
        let callbacks = self.context.cb.clone();
        callbacks.trigger_callbacks(&mut self.context, &mut self.core_app);
        self.context.cb.clear_triggered();

        update_animations(dt, &mut self.context);
        update_target_attributes(dt, &mut self.context);
        draw_scene(&mut self.context, Rc::clone(&self.gl), &self.program, &self.program_color);
        //panic!("one loop");
    }
}


pub fn send_events_from_context(context: &mut Context, events: &Vec<Event>, this_frame: &mut Vec<Box<dyn VisualNode>>) {
    for event in events {
        for vn in 0..this_frame.len() {
            web_sys::console::log_1(&"sending animation event:".into());
            this_frame.get_mut(vn).unwrap().event_handler(context, &event);
        }
    }
}

pub fn send_events(context: &mut Context, events: Rc<RefCell<Handler>>, this_frame: &mut Vec<Box<dyn VisualNode>>) {
    let nodes =  context.nodes.clone();

    let event = events.borrow().event;
    if event == Event::None {
        return
    }

    let cx = context; 
    for node in nodes.iter().rev() {  //todo iteraton over both nodes and visual nodes.
        for vn in 0..this_frame.len() {
            let mut visual_node = this_frame.get_mut(vn).unwrap();
            if visual_node.get_uuid() == node.owner {
                web_sys::console::log_2(&"sending to node ".into(), &node.uuid.to_string().into());
                send_event(Rc::clone(&events), cx, node.position(), &mut visual_node)
            }
        }
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

/*                web_sys::console::log_1(&"event".into());
                web_sys::console::log_1(&event.x.to_string().into());
                web_sys::console::log_1(&event.y.to_string().into());
                web_sys::console::log_1(&"node from".into());
                web_sys::console::log_1(&xy.0.to_string().into());
                web_sys::console::log_1(&xy.1.to_string().into());
                web_sys::console::log_1(&"node to".into());
                web_sys::console::log_1(&xy.2.to_string().into());
                web_sys::console::log_1(&xy.3.to_string().into());
*/
                if xy.0 < x as f32 && xy.2 > x as f32 && xy.1 < y as f32 && xy.3 > y as f32 {
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

pub fn create(mut context: &mut Context, core_app: &mut CoreApp) {
    let slider = Slider::new(&mut context, 500.0, 500.0, 1.0);
    let image_view = ImageViewBuilder::builder().x(0.0).y(0.0).width(3264.0).height(2448.0).opacity(1.0).image("IMG_20160408_164451.jpg").build(&mut context);

    let container = ContainerBuilder::builder().x(500.0).height(400.0).width(400.0).color((1.0,1.0,0.0)).opacity(0.5).clip(true).build(&mut context);
    let container2 = ContainerBuilder::builder().x(20.0).y(20.0).width(400.0).height(400.0).color((0.0,1.0,1.0)).opacity(0.5).build(&mut context);
    let label = Label::new(context, 5.0, 5.0, "Test label");
    context.add_child_to(context.root.unwrap(), container.get_node_uuid());
    context.add_child_to(context.root.unwrap(), slider.get_node_uuid());
    container.add_child(&mut context, container2.get_node_uuid());
    
    
    let scroll_view = ScrollView::new(&mut context, 0.0, 0.0, 0.0, 0.0, 0.5, 300.0, 300.0, (1.0,0.0,1.0), );
    scroll_view.add_content(&mut context, image_view.get_node_uuid());
    context.add_child_to(context.root.unwrap(), scroll_view.get_node_uuid());

    let button = ButtonPrivate::new(&mut context, "Button", 150.0, 5.0, 0.5);
    core_app.names.insert("Bild".to_owned(), image_view.get_uuid());

    context.add_child_to(context.root.unwrap(), button.get_node_uuid());
    context.add_child_to(context.root.unwrap(), label.get_node_uuid());

    context.cb.add_subscriber(button.this, on_button_pressed);
    context.cb.add_subscriber(slider.this, on_scroll);

    core_app.visual_nodes.push(Box::new(image_view) as Box<dyn VisualNode>);
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

pub fn on_scroll(mut cx: &mut Context, core_app: &mut CoreApp, event: Event) {
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
