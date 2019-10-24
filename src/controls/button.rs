use crate::rnd::texture_unit::*;
use crate::rnd::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::draw::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animator::*;
use serde::{Serialize, Deserialize};

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
    event_handler: Option<Box<Fn(&MouseEvent)>>,
}


impl Button {
    pub fn new(x: f32, y: f32, opacity: f32) -> Button { 
        let node = Node { x, y, opacity };
        let pressed = false;
        let animator = Option::None; 
        let clicks = vec![];    
        let event_handler = Option::None; 
        Button { node, pressed, animator, clicks, event_handler } 
    }

    pub fn setClickHandler(&mut self, handler : Option<Box<Fn(&MouseEvent)>>) {
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
        if event.event == MouseEvent::Up {
            self.pressed = false;
        } else if event.event == MouseEvent::Down {
            self.pressed = true; 
        }        
        match self.event_handler {
            Some(ref handler) => {
                web_sys::console::log_1(&"calling calback".into());   
                handler(&event.event);
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