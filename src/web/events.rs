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
        let x = event.client_x() as u16;
        let y = event.client_y() as u16;
        let mouse_event = Event::Mouse(Mouse::new(x, y, MouseEvent::Down));
        handler.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

pub fn attach_mouse_up_handler(
    canvas: &web_sys::HtmlCanvasElement,
    events: Rc<RefCell<Handler>>,
) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x() as u16;
        let y = event.client_y() as u16;
        let mouse_event = Event::Mouse(Mouse::new(x, y, MouseEvent::Up));
        events.borrow_mut().set_event(mouse_event);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
