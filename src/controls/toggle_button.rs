use std::any::Any;
use crate::render::texture_unit::*;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::animation::ease::*;
extern crate erased_serde;
use uuid::Uuid;
use crate::application::context::*;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
enum ToggleButtonState {
    On,
    Off,
    ToOn,
    ToOff,
}


pub struct ToggleButtonPrivate {
    this: Uuid,
    background_uuid: Uuid,
    toggle_uuid: Uuid,
    value: bool,
    on_animation_uuid: Uuid,
    off_animation_uuid: Uuid,
    state: ToggleButtonState, 
    pub closure: Option<Box<dyn FnMut(&mut Context) >>,
}

impl ToggleButtonPrivate {
    pub fn new(cx: &mut Context, x: f32, y: f32, opacity: f32) -> ToggleButtonPrivate { 
        let this = Uuid::new_v4();
        let value = false;
        let background = ToggleButtonPrivate::create_background(this, cx, x, y, opacity);
        let toggle = ToggleButtonPrivate::create_toggle(this,cx, x, y, opacity);
        let state = ToggleButtonState::Off;

        let mut on_animation = Animation::new(toggle.uuid, Attribute::X);
        on_animation.duration = 250.0;
        on_animation.start_value = 0.0;
        on_animation.end_value = 70.0;
        on_animation.easing = Ease::OutCubic;

        let mut off_animation = Animation::new(toggle.uuid, Attribute::X);
        off_animation.duration = 250.0;
        off_animation.start_value = 70.0;
        off_animation.end_value = 0.0;
        off_animation.easing = Ease::OutCubic;

        let background_uuid = background.uuid;
        let toggle_uuid = toggle.uuid;

        let on_animation_uuid = on_animation.uuid;
        let off_animation_uuid = off_animation.uuid;

        let closure = Option::None;

        let toggle_button_private = ToggleButtonPrivate { this, background_uuid, toggle_uuid, value, on_animation_uuid, off_animation_uuid, state, closure }; 

        cx.nodes.push(background);
        cx.nodes.push(toggle);

        cx.animations.push(on_animation);
        cx.animations.push(off_animation);
        toggle_button_private
    }

    fn create_background(this: Uuid, cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width = 107.0;
        let height = 36.0;
        let texture = TextureUnit::ToggelBackground;
        let mut node = Node::new(this, cx, x, y, width, height);
        node.texture = Some(Node::create_texture(cx, "/assets/bg.png"));
        node
    }

    fn create_toggle(this: Uuid, cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width = 30.0;
        let height = 30.0;
        let x = x + 3.0;
        let y = y + 3.0;
        let texture = TextureUnit::Toggle;
        let mut node = Node::new(this, cx, x, y, width, height);
        node.opacity = opacity;
        node.texture = Some(Node::create_texture(cx, "/assets/grey.png"));
        node
    }

    pub fn to_toggle_button_private(s: &dyn Any) -> Option<&ToggleButtonPrivate>{
        if let Some(toggle_button) = s.downcast_ref::<ToggleButtonPrivate>() {
            Some(&toggle_button)
        } else {
            None
        }
    }

    pub fn on_button_pressed(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set on".into());
        match self.state {
            ToggleButtonState::Off => self.play_on_animation(cx),
            ToggleButtonState::On => self.play_off_animation(cx),
            ToggleButtonState::ToOff => (),
            ToggleButtonState::ToOn => (),
        }
    }

    pub fn on_animation_ended(&mut self, cx: &mut Context) {

        web_sys::console::log_1(&"on_animation_ended".into());
        match self.state {
            ToggleButtonState::Off => (),
            ToggleButtonState::On => (),
            ToggleButtonState::ToOff => self.set_off(cx),
            ToggleButtonState::ToOn => self.set_on(cx),
             _ => ()
        }
    }
    
    pub fn set_on(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set on".into());
        self.state = ToggleButtonState::On;
        let texture = Some(Node::create_texture(cx, "/assets/blue.png"));
        let node = cx.get_node(self.toggle_uuid).unwrap();
        node.texture = texture;
    }

    fn set_off(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set off".into());
        self.state = ToggleButtonState::Off;
        let texture = Some(Node::create_texture(cx, "/assets/grey.png"));
        let node = cx.get_node(self.toggle_uuid).unwrap();
        node.texture = texture;
    }

    fn play_off_animation(&mut self, context: &mut Context) {
        web_sys::console::log_1(&"play off".into());
        self.state = ToggleButtonState::ToOff;
        let on_animation = context.get_animation(self.off_animation_uuid);
        on_animation.unwrap().play();
    }

    fn play_on_animation(&mut self, context: &mut Context) {
        web_sys::console::log_1(&"play on".into());
        self.state = ToggleButtonState::ToOn;
        let on_animation = context.get_animation(self.on_animation_uuid);
        on_animation.unwrap().play();
        web_sys::console::log_1(&"playing".into());
    }
    
}

impl VisualNode for ToggleButtonPrivate {

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, mut cx: &mut Context, message: &Event) -> bool{
        web_sys::console::log_1(&"got event".into());
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                    self.on_button_pressed(cx);
                    if self.closure.is_some() {
                        let callback : &mut Box<dyn FnMut(&mut Context)> = self.closure.as_mut().unwrap();
                        web_sys::console::log_1(&"Calling callback".into());
                        callback(&mut cx);
                    }
                }
                return true;
            },
            Event::Message(message) => {
                match message {
                    Message::AnimationEnded(uuid) => {
                        if *uuid == self.on_animation_uuid || *uuid == self.off_animation_uuid {
                            self.on_animation_ended(cx)
                        } 
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        web_sys::console::log_1(&"false".into());
        return false;
    }

    fn get_uuid(&self) -> uuid::Uuid { 
        self.this
    }

}
