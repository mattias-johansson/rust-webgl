use std::any::Any;
use std::rc::Rc;
use std::rc::Weak;
use crate::render::texture_unit::*;
use crate::render::draw::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::animation::ease::*;
extern crate erased_serde;
use std::cell::RefCell;
use crate::events::handler::Handler;
use uuid::Uuid;
use crate::application::context::*;

#[derive(PartialEq, Clone, Copy)]
enum ToggleButtonState {
    On,
    Off,
    ToOn,
    ToOff,
}

pub struct ToggleButtonPrivate {
    background: Node,
    toggle: Node,
    value: bool,
    on_animation: Animation,
    off_animation: Animation,
    state: ToggleButtonState,
}

impl ToggleButtonPrivate {
    pub fn new(cx: &mut Context, x: f32, y: f32, opacity: f32) -> ToggleButtonPrivate { 
        let value = false;
        let background = ToggleButtonPrivate::create_background(cx, x, y, opacity);
        let toggle = ToggleButtonPrivate::create_toggle(cx, x, y, opacity);
        let state = ToggleButtonState::Off;

        let mut on_animation = Animation::new(toggle.uuid, Attribute::X);
        on_animation.duration = 2000.0;
        on_animation.start_value = 0.0;
        on_animation.end_value = 74.0;
        on_animation.easing = Ease::InElastic;

        let mut off_animation = Animation::new(toggle.uuid, Attribute::X);
        off_animation.duration = 2000.0;
        off_animation.start_value = 74.0;
        off_animation.end_value = 0.0;
        off_animation.easing = Ease::InElastic;

        ToggleButtonPrivate { background, toggle, value, on_animation, off_animation, state } 
    }

    fn create_background(cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width = 107.0;
        let height = 36.0;
        let texture = TextureUnit::ToggelBackground;
        let mut node = Node::new(cx, x, y, width, height);
        node.texture = texture;
        node
    }

    fn create_toggle(cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width = 30.0;
        let height = 30.0;
        let texture = TextureUnit::Toggle;
        let mut node = Node::new(cx, x, y, width, height);
        node.texture = texture;
        node
    }

    pub fn to_toggle_button_private(s: &dyn Any) -> Option<&ToggleButtonPrivate>{
        if let Some(toggle_button) = s.downcast_ref::<ToggleButtonPrivate>() {
            Some(&toggle_button)
        } else {
            None
        }
    }

    pub fn on_button_pressed(&mut self) {
        match self.state {
            ToggleButtonState::Off => (),
            ToggleButtonState::On => (),
            ToggleButtonState::ToOff => self.set_off(),
            ToggleButtonState::ToOn => self.set_on(),
        }
    }

    pub fn on_animation_ended(&mut self) {
        match self.state {
            ToggleButtonState::Off => (),
            ToggleButtonState::On => (),
            ToggleButtonState::ToOff => self.state = ToggleButtonState::Off,
            ToggleButtonState::ToOn => self.state = ToggleButtonState::On,
        }
    }
    
    pub fn set_on(&mut self) {
        self.toggle.texture = TextureUnit::ToggleActive;
        self.on_animation.play();

    }

    fn set_off(&mut self) {
        self.toggle.texture = TextureUnit::Toggle;
        self.off_animation.play();
    }
}

impl VisualNode for ToggleButtonPrivate {


    fn as_any(&self) -> &dyn Any {
        self
    }
/*
    fn draw(&mut self, context: &WebGlRenderingContext, program: &WebGlProgram, time: f32) {
        if self.animator.pressed {
            self.animator.set_start_time(time);
        }
        if self.value {        
            render(&context, &program, 107.0, 36.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::ToggelBackground);
            let end = self.node.x + 74.0;
            if self.animator.running {
                let time = (time - self.animator.start_time) / self.animator.duration;

                let x = self.animator.easing.map((time) as f32);
                let start = self.node.x + 3.0;
//                let js: JsValue = x.into();
//                web_sys::console::log_1(&js);

                let x = x * (end - start);
                render(&context, &program, 30.0, 30.0, start + x, self.node.y + 3.0, TextureUnit::ToggleActive);
                if x == 1.0 {
                    self.animator.set_running(false);
                }
            } else {
                render(&context, &program, 30.0, 30.0, end, self.node.y + 3.0, TextureUnit::ToggleActive);
            }
        } else {
            render(&context, &program, 107.0, 36.0, self.node.x + 0.0, self.node.y + 0.0, TextureUnit::ToggelBackground);
            let end = self.node.x + 3.0;
            if self.animator.running {
                let time = (time - self.animator.start_time) / self.animator.duration;

                let x = self.animator.easing.map((time) as f32);
                let start = self.node.x + 74.0;
//                let js: JsValue = x.into();
//                web_sys::console::log_1(&js);

                let x = x * (end - start);

                render(&context, &program, 30.0, 30.0, start + x, self.node.y + 3.0, TextureUnit::Toggle);
            } else {
                render(&context, &program, 30.0, 30.0, end, self.node.y + 3.0, TextureUnit::Toggle);
            }
        }
    }
*/

    fn event_handler(&mut self, message: &Event) {
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                    self.on_button_pressed();
                }
            },
            Event::Message(message) => {
                match message {
                    AnimationEnded => self.on_animation_ended(),
                    _ => (),
                }
            },
            _ => ()
        }
    }
/*
    fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 107.0, self.node.y + 36.0)
    }
*/

}
