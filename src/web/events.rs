use web_sys::Worker;
use crate::ApplicationEvents;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;

use crate::events::handler::*;
use crate::events::mouse::*;


pub fn attach_mouse_down_handler(
    canvas: &web_sys::HtmlCanvasElement,
    handler: Rc<RefCell<Handler>>,
) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x() as i32;
        let y = event.client_y() as i32;
        let mouse_event = Event::Mouse(Mouse::new(x, y, MouseEvent::Down));
        handler.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

pub fn attach_mouse_up_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,
) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x() as i32;
        let y = event.client_y() as i32;
        let mouse_event = Event::Mouse(Mouse::new(x, y, MouseEvent::Up));
        events.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}


pub fn attach_mouse_move_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        if event.movement_x() != 0 || event.movement_y() != 0 {
            //web_sys::console::log_1(&"mouse_move_handler".into());
            event.prevent_default();
            let x = event.client_x() as i32;
            let y = event.client_y() as i32;
            let movement_x = event.movement_x() as i32;
            let movement_y = event.movement_y() as i32;        
            let mouse_event = Event::Mouse(Mouse::new_2(x, y, movement_x, movement_y, MouseEvent::Move));

            events.borrow_mut().set_event(mouse_event);
        }
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
/*
pub fn attach_mouse_wheel_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,) -> Result<(), JsValue> {
    let handler = move |event: web_sys::WheelEvent| {
        event.prevent_default();
        let zoom_amount = event.delta_y() / 50.;
        events.borrow_mut().set_event(&Msg::Zoom(zoom_amount as f32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
*/
pub fn attach_touch_start_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x() as i32;
        let y = touch.client_y() as i32;        
        let mouse_event = Event::Mouse(Mouse::new(x, y, MouseEvent::Down));
        events.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchstart", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

pub fn attach_touch_move_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        event.prevent_default();
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x() as i32;
        let y = touch.client_y() as i32;        
        let mouse_event = Event::Mouse(Mouse::new(x, y, MouseEvent::Move));
        events.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchmove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

pub fn attach_touch_end_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,) -> Result<(), JsValue> {
    let handler = move | event: web_sys::TouchEvent| {
    let touch = event.touches().item(0).expect("First Touch");
    let x = touch.client_x() as i32;
    let y = touch.client_y() as i32;        
    let mouse_event = Event::Mouse(Mouse::new(x, y, MouseEvent::Up));
    events.borrow_mut().set_event(mouse_event);

    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("touchend", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

pub fn add_on_message_handler(
    worker: &Worker,
    events: Rc<RefCell<ApplicationEvents>>) -> Result<(), JsValue> {
      
//        web_sys::console::debug_1(&"pub fn add_on_message_handler".into());
        let handler = move |event: web_sys::MessageEvent| {
            let data = event.data();  
            let event_id = event.last_event_id();
                match events.try_borrow_mut() {
                    Ok(mut mut_events) => { 
                        mut_events.add_event(data.as_string().unwrap()); 
//                        web_sys::console::debug_2(&"message_data".into(), &data.as_string().unwrap().into());
//                        web_sys::console::debug_2(&"message_event_id".into(), &event_id.into());
                    },
                    Err(err) => { web_sys::console::error_2(&"Cannot borrow events as mut".into(), &err.to_string().into()); }
                }
            };

            let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

            worker.set_onmessage(Some(handler.as_ref().unchecked_ref()));
            handler.forget();
        Ok(())
    }
