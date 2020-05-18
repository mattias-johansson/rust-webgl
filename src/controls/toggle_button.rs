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
    background_uuid: Uuid,
    toggle_uuid: Uuid,
    value: bool,
    on_animation_uuid: Uuid,
    off_animation_uuid: Uuid,
    state: ToggleButtonState, 
    pub closure: Option<Box<dyn FnMut() >>,
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
        on_animation.easing = Ease::Lin;

        let mut off_animation = Animation::new(toggle.uuid, Attribute::X);
        off_animation.duration = 2000.0;
        off_animation.start_value = 74.0;
        off_animation.end_value = 0.0;
        off_animation.easing = Ease::Lin;

        let background_uuid = background.uuid;
        let toggle_uuid = toggle.uuid;

        let on_animation_uuid = on_animation.uuid;
        let off_animation_uuid = off_animation.uuid;

        let closure = Option::None;

        let toggleButtonPrivate = ToggleButtonPrivate { background_uuid, toggle_uuid, value, on_animation_uuid, off_animation_uuid, state, closure }; 

        cx.nodes.push(background);
        cx.nodes.push(toggle);

        cx.animations.push(on_animation);
        cx.animations.push(off_animation);
        toggleButtonPrivate
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
        let x = x + 3.0;
        let y = y + 3.0;
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

    pub fn on_button_pressed(&self, cx: &mut Context) {
        web_sys::console::log_1(&"set on".into());
        match self.state {
            ToggleButtonState::Off => self.play_off_animation(cx),
            ToggleButtonState::On => self.play_on_animation(cx),
            ToggleButtonState::ToOff => self.set_off(cx),
            ToggleButtonState::ToOn => self.set_on(cx),
        }
    }

    pub fn on_animation_ended(&self) {
        match self.state {
            ToggleButtonState::Off => (),
            ToggleButtonState::On => (),
//            ToggleButtonState::ToOff => self.state = ToggleButtonState::Off,
///            ToggleButtonState::ToOn => self.state = ToggleButtonState::On,
             _ => ()
        }
    }
    
    pub fn set_on(&self, cx: &mut Context) {
        web_sys::console::log_1(&"set on".into());
        let node = cx.get_node(self.toggle_uuid).unwrap();
        node.texture = TextureUnit::ToggleActive;
    }

    fn set_off(&self, cx: &mut Context) {
        web_sys::console::log_1(&"set off".into());
        let node = cx.get_node(self.toggle_uuid).unwrap();
        node.texture = TextureUnit::Toggle;
    }

    fn play_off_animation(&self, context: &mut Context) {
        let on_animation = context.get_animation(self.on_animation_uuid);
        on_animation.unwrap().play();
    }

    fn play_on_animation(&self, context: &mut Context) {
        let on_animation = context.get_animation(self.on_animation_uuid);
        on_animation.unwrap().play();
    }
    
}

impl <'a> VisualNode for ToggleButtonPrivate {


    fn as_any(&mut self) -> &mut dyn Any {
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

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool{
        web_sys::console::log_1(&"got event".into());
        match message {
            Event::Mouse(event) => {
                web_sys::console::log_1(&"pressed".into());
                if event.event == MouseEvent::Up {
                    web_sys::console::log_1(&"pressed".into());
                    self.on_button_pressed(cx);
                    let callback : &mut Box<dyn FnMut()> = &mut self.closure.as_mut().unwrap();
                    callback();
                }
                return true;
            },
            Event::Message(message) => {
                match message {
                    AnimationEnded => self.on_animation_ended(),
                    _ => (),
                }
            },
            _ => ()
        }

        web_sys::console::log_1(&"false".into());
        return false;
    }
/*
    fn position(&self) -> (f32, f32, f32, f32) {
        (self.node.x, self.node.y, self.node.x + 107.0, self.node.y + 36.0)
    }
*/

}
