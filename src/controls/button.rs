use core::cell::RefCell;
use std::rc::Weak;
use crate::events::handler::Handler;
use crate::events::click_handler::ClickHandler;
use std::rc::Rc;
use crate::rnd::texture_unit::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::draw::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animator::*;
use serde::{Serialize, Deserialize};
use crate::page::*;

extern crate erased_serde;

#[derive(Serialize, Deserialize)]
pub struct Click {
    handled: bool,
}

pub struct Button {
    node: Node,
    pressed: bool,
    animator: Option<Animator>,
    clicks: Vec<Click>,
    event_handler: Option<Box<Fn(&mut Page, &MouseEvent)>>,
    callback: Option<Weak<dyn ClickHandler<MouseEvent>>>,
}


impl Button {
    pub fn new(x: f32, y: f32, opacity: f32, parent: Option<Rc<Node>>) -> Button { 
        let node = Node { x, y, opacity, parent };
        let pressed = false;
        let animator = Option::None; 
        let clicks = vec![];    
        let event_handler = Option::None; 
        let callback = Option::None;
        Button { node, pressed, animator, clicks, event_handler, callback } 
    }
/*
    pub fn set_click_handler(&mut self, handler : Option<ClickHandler>) {
        self.event_handler = handler;
    }
    */

    pub fn set_callback(&mut self, callback: Option<Weak<dyn ClickHandler<MouseEvent>>>) {
        self.callback = callback;
    }
    pub fn set_click_handler(&mut self, handler : Option<Box<Fn(&mut Page, &MouseEvent)>>) {
        self.event_handler = handler;
     }
}

impl Draw for Button {
    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {
        if self.pressed {
            render(&context, &program, 145.0, 34.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::ButtonPressed);
        } else {
            render(&context, &program, 145.0, 34.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::Button);    
        }
    }

    fn event(&mut self, event: &Mouse) {
        web_sys::console::log_1(&"button handling event".into());   
        if event.event == MouseEvent::Up {
            self.pressed = false;
        } else if event.event == MouseEvent::Down {
            self.pressed = true; 
        }        
        match self.event_handler {
            Some(ref handler) => {
                web_sys::console::log_1(&"calling calback".into());   
  //              handler(&event.event);
            },
            None => {
                web_sys::console::log_1(&"Calback is none".into());   
            },
        }
    }

   fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 145.0, self.node.y + 34.0)
    }
}
