use crate::render::word::Word;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::any::Any;

use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::application::context::*;
use crate::animation::ease::*;
use uuid::Uuid;
use serde::*;
use crate::globals::messaging::*;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
enum ButtonState {
    NotPressed,
    Pressed,
    ToPressed,
    ToNotPressed,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Button {
    this: Uuid,
    text: String, 
    x: f32, 
    y: f32, 
    opacity: f32,
    state: ButtonState,
}

//TODO Nedd to rewrite as composite control. Need to have a container node as "node"

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ButtonPrivate {
    pub this: Uuid,
    node: Uuid,
    pressed: bool,
    state: ButtonState,
    on_animation_uuid: Uuid,
    off_animation_uuid: Uuid,
}

impl ButtonPrivate {

    pub fn from_public(cx: &mut Context, public: Button) -> ButtonPrivate {
        ButtonPrivate::new(public.this, cx, public.text.as_str(), public.x, public.y, public.opacity)
    }

    pub fn new(this: Uuid, cx: &mut Context, text: &str, x: f32, y: f32, opacity: f32) -> ButtonPrivate { 
        let pressed = false;

        let mut node = Node::new(this, x, y, 145.0, 34.0);
        node.opacity = 0.0;
        let node_uuid_parent = node.uuid;
        cx.nodes.push(node);

        let node = ButtonPrivate::create(this, cx, 0.0, 0.0, opacity);
        let node_uuid = node.uuid;
        cx.nodes.push(node);
        cx.add_child_to(node_uuid_parent, node_uuid);

        let state = ButtonState::NotPressed; 
        
        let node_pressed = ButtonPrivate::create_pressed(this, cx, 0.0, 0.0, opacity);
        let node_pressed_uuid = node_pressed.uuid;
        cx.nodes.push(node_pressed);
        cx.add_child_to(node_uuid_parent, node_pressed_uuid);

        let node_uuid = ButtonPrivate::create_text(cx, text, 0.0, 0.0);
        cx.add_child_to(node_uuid_parent, node_uuid);

        let mut on_animation = Animation::new(node_uuid, Attribute::OPACITY);
        on_animation.duration = 100.0;
        on_animation.start_value = 0.0;
        on_animation.end_value = 1.0;
        on_animation.easing = Ease::Lin;

        let mut off_animation = Animation::new(node_pressed.uuid, Attribute::OPACITY);
        off_animation.duration = 100.0;
        off_animation.start_value = 1.0;
        off_animation.end_value = 0.0;
        off_animation.easing = Ease::Lin;

        let on_animation_uuid = on_animation.uuid;
        let off_animation_uuid = off_animation.uuid;

        cx.animations.push(on_animation);
        cx.animations.push(off_animation);

        ButtonPrivate { this, node: node_uuid_parent, pressed, state, on_animation_uuid, off_animation_uuid } 
    }

    pub fn to_button_private(s: &dyn Any) -> Option<&ButtonPrivate>{
        if let Some(button) = s.downcast_ref::<ButtonPrivate>() {
            Some(&button)
        } else {
            None
        }
    }

    pub fn create(this: Uuid, cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width =  145.0;
        let height = 34.0;
        let mut node = Node::new(this, x, y, width, height);
        node.opacity = 1.0;
        node.texture = Some(Node::create_texture(cx, "/assets/button.png"));
        node
    }

    pub fn create_pressed(this: Uuid, cx: &mut Context, x: f32, y: f32, opacity: f32) -> Node {
        let width =  145.0;
        let height = 34.0;
        let mut node = Node::new(this, x, y, width, height);
        node.opacity = 0.0;
        node.texture = Some(Node::create_texture(cx, "/assets/button_pressed.png"));
        node
    }

    pub fn create_text(cx: &mut Context, text: &str, x: f32, y: f32)  -> Uuid{
        let this = Uuid::new_v4();
        let mut node = Node::new(this, x, y, 0.0, 0.0);
        let node_uuid_parent = node.uuid;
        let text = String::from(text);
        let mut word = Word::default();
        word.create_char_points_for_text(&text);
        let mut char_iter = text.chars();
        let mut advance: f32 = 0.0;
        while let Some(c) = char_iter.next() {
            let mut node = Node::new(this, advance, y, 100.0, 100.0);
            node.text = true;
            let node_uuid = node.uuid;
            cx.vertices.insert(node.uuid, word.get_char_points_for_char(&(c as usize)).unwrap().to_vec());
            advance = advance + (word.get_advance_for_char(c as usize) * 0.009);   
            
            web_sys::console::log_1(&advance.to_string().into());         
            cx.nodes.push(node);
            cx.add_child_to(node_uuid_parent, node_uuid);
        }
        node.x = (145.0 - advance) / 2.0;
        node.y = 10.0;
        cx.nodes.push(node);
        node_uuid_parent
    }

    pub fn on_button_pressed(&mut self, cx: &mut Context) {
//        web_sys::console::log_1(&"set on".into());
        match self.state {
            ButtonState::NotPressed => self.play_on_animation(cx),
            ButtonState::Pressed => self.play_off_animation(cx),
            ButtonState::ToNotPressed => (),
            ButtonState::ToPressed => (),
        }
    }

    pub fn on_animation_ended(&mut self, cx: &mut Context) {
//        web_sys::console::log_1(&"on_animation_ended".into());
        match self.state {
            ButtonState::NotPressed => (),
            ButtonState::Pressed => (),
            ButtonState::ToNotPressed  => self.set_off(cx),
            ButtonState::ToPressed  => self.set_on(cx),
             _ => ()
        }
    }

    pub fn set_on(&mut self, cx: &mut Context) {
//        web_sys::console::log_1(&"set Pressed".into());
        self.state = ButtonState::Pressed;
        let on_animation = cx.get_animation(self.on_animation_uuid);
        on_animation.unwrap().start_value = 1.0;
        let on_animation = cx.get_animation(self.on_animation_uuid);
        on_animation.unwrap().end_value = 0.0;

        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().start_value = 0.0;
        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().end_value = 1.0;

    }

    fn set_off(&mut self, cx: &mut Context) {
//        web_sys::console::log_1(&"set NotPressed".into());
        self.state = ButtonState::NotPressed;
        let on_animation = cx.get_animation(self.on_animation_uuid);
        on_animation.unwrap().start_value = 0.0;
        let on_animation = cx.get_animation(self.on_animation_uuid);
        on_animation.unwrap().end_value = 1.0;

        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().start_value = 1.0;
        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().end_value = 0.0;
    }

    fn play_off_animation(&mut self, context: &mut Context) {
//        web_sys::console::log_1(&"play ToNotPressed".into());
        self.state = ButtonState::ToNotPressed;
        let off_animation = context.get_animation(self.off_animation_uuid);
        off_animation.unwrap().play();
        let on_animation = context.get_animation(self.on_animation_uuid);
        on_animation.unwrap().play();
    }

    fn play_on_animation(&mut self, context: &mut Context) {
//        web_sys::console::log_1(&"play ToPressed".into());
        self.state = ButtonState::ToPressed;
        let on_animation = context.get_animation(self.on_animation_uuid);
        on_animation.unwrap().play();
        let off_animation = context.get_animation(self.off_animation_uuid);
        off_animation.unwrap().play();
    }
}

impl VisualNode for ButtonPrivate {

    fn get_node_uuid(&self) -> Uuid {
        self.node
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool {
//        web_sys::console::log_1(&"got event".into());
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                    web_sys::console::log_1(&"mouse up".into());
                    self.pressed = false;
                    self.on_button_pressed(cx);
                    cx.cb.add_trigged(self.this, Event::None);
                    let _  = cx.messaging.send_message(MessageType::ValueUpdated(self.this, "onClicked".to_owned(), "true".to_owned()));
                } else if event.event == MouseEvent::Down {
                    self.pressed = true; 
                    self.on_button_pressed(cx);
                }      
                return true;
            },
            Event::Message(message) => {
                match message {
                    Message::AnimationEnded(_uuid) => self.on_animation_ended(cx),
                    //TODO Two animations are ending. Handle that
                    _ => ()
                }
            },
            _ => ()
        }
//        web_sys::console::log_1(&"false".into());
        return false;
    }

    fn get_uuid(&self) -> uuid::Uuid { 
        self.this
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
