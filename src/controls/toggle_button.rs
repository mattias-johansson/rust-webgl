use uimsg::MessageType;
use std::any::Any;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::animation::ease::*;
use uuid::Uuid;
use crate::application::context::*;
use serde::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum ToggleButtonState {
    On,
    Off,
    ToOn,
    ToOff,
}

#[derive(Clone, Copy)]
pub struct ToggleButtonPrivate {
    this: Uuid,
    background_uuid: Uuid,
    toggle_uuid: Uuid,
    on_animation_uuid: Uuid,
    off_animation_uuid: Uuid,
    state: ToggleButtonState, 
    pub callback: Option<fn(&mut Context)>,
}

impl ToggleButtonPrivate {


    pub fn from_public(cx: &mut Context, public: ToggleButton) -> ToggleButtonPrivate {
        ToggleButtonPrivate::new(public.this, cx, public.x, public.y, public.opacity)
    }

    pub fn new(this: Uuid, cx: &mut Context, x: f32, y: f32, opacity: f32) -> ToggleButtonPrivate { 
    
        let background = ToggleButtonPrivate::create_background(this, cx, x, y, opacity);
        let toggle = ToggleButtonPrivate::create_toggle(this,cx);
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

        cx.add_child_to(background_uuid, toggle_uuid);

        let on_animation_uuid = on_animation.uuid;
        let off_animation_uuid = off_animation.uuid;

        let callback = Option::None;

        let toggle_button_private = ToggleButtonPrivate { this, background_uuid, toggle_uuid, on_animation_uuid, off_animation_uuid, state, callback }; 

        cx.nodes.push(background);
        cx.nodes.push(toggle);

        cx.animations.push(on_animation);
        cx.animations.push(off_animation);
        toggle_button_private
    }

    fn create_background(this: Uuid, cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width = 107.0;
        let height = 36.0;
        let mut node = Node::new(this, x, y, width, height);
        node.set_opacity(opacity);
        node.set_texture(Some(Node::create_texture(cx, "/assets/bg.png")));
        node
    }

    fn create_toggle(this: Uuid, cx: &mut Context) -> Node {
        let width = 30.0;
        let height = 30.0;
        let x = 3.0;
        let y = 3.0;
        let mut node = Node::new(this, x, y, width, height);
        node.set_texture(Some(Node::create_texture(cx, "/assets/grey.png")));
        node
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
        match self.state {
            ToggleButtonState::Off => (),
            ToggleButtonState::On => (),
            ToggleButtonState::ToOff => self.set_off(cx),
            ToggleButtonState::ToOn => self.set_on(cx)
        }
    }
    
    pub fn set_on(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set on".into());
        self.state = ToggleButtonState::On;
        let texture = Some(Node::create_texture(cx, "/assets/blue.png"));
        let node = cx.get_node(self.toggle_uuid).unwrap();
        node.set_texture(texture);
                {
            let mut dirty = cx.dirty.borrow_mut();
            *dirty = true;
        }
        let _  = cx.messaging.send_message(MessageType::ValueUpdated(self.this, "checked".to_owned(), "true".to_owned()));
    }

    fn set_off(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set off".into());
        self.state = ToggleButtonState::Off;
        let texture = Some(Node::create_texture(cx, "/assets/grey.png"));
        let node = cx.get_node(self.toggle_uuid).unwrap();
        node.set_texture(texture);
        {
            let mut dirty = cx.dirty.borrow_mut();
            *dirty = true;
        }
        let _  = cx.messaging.send_message(MessageType::ValueUpdated(self.this, "checked".to_owned(), "false".to_owned()));
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

    fn get_node_uuid(&self) -> Uuid {
        self.background_uuid
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, mut cx: &mut Context, message: &Event, _target: Uuid) -> bool{

//        web_sys::console::log_1(&"got event".into());
        
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                    self.on_button_pressed(cx);
                    if self.callback.is_some() {
                        let callback = self.callback.unwrap();
                        web_sys::console::log_1(&"Calling callback".into());
                        callback(&mut cx);
                    }
                }
//                return true;
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

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct ToggleButton {
    this: Uuid,
    text: String, 
    x: f32, 
    y: f32, 
    opacity: f32,
}