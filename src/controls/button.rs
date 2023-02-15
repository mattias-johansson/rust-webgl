use crate::render::word::Word;
use std::any::Any;

use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::events::mouse::*;
use crate::animation::animation::*;
use crate::application::context::*;
use crate::animation::ease::*;
use uuid::Uuid;
use serde::*;

use uimsg::MessageType;
use uimsg::Button;
use uimsg::ButtonState;


#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ButtonPrivate {
    pub this: Uuid,
    node: Uuid,
    pressed: bool,
    state: ButtonState,
    off_animation_uuid: Uuid,
}

impl ButtonPrivate {

    pub fn from_public(cx: &mut Context, public: Button) -> ButtonPrivate {
        ButtonPrivate::new(public.this(), cx, public.text().as_str(), public.x(), public.y(), public.opacity())
    }

    pub fn new(this: Uuid, cx: &mut Context, text: &str, x: f32, y: f32, opacity: f32) -> ButtonPrivate { 
        let pressed = false;

        let mut node = Node::new(this, x, y, 145.0, 34.0);
        node.set_opacity(opacity);
        let node_uuid_parent = node.uuid;
        cx.nodes.push(node);

        let node = ButtonPrivate::create(this, cx, 0.0, 0.0);
        let node_uuid = node.uuid;
        cx.nodes.push(node);
        cx.add_child_to(node_uuid_parent, node_uuid);

        let state = ButtonState::NotPressed; 
        
        let node_pressed = ButtonPrivate::create_pressed(this, cx, 0.0, 0.0);
        let node_pressed_uuid = node_pressed.uuid;
        cx.nodes.push(node_pressed);
        cx.add_child_to(node_uuid_parent, node_pressed_uuid);

        let node_uuid = ButtonPrivate::create_text(cx, text, 0.0, 0.0);
        cx.add_child_to(node_uuid_parent, node_uuid);

        let mut off_animation = Animation::new(node_pressed.uuid, Attribute::OPACITY);
        off_animation.duration = 100.0;
        off_animation.start_value = 0.0;
        off_animation.end_value = 1.0;
        off_animation.easing = Ease::Lin;

        let off_animation_uuid = off_animation.uuid;
        cx.animations.push(off_animation);


        ButtonPrivate { this, node: node_uuid_parent, pressed, state, off_animation_uuid } 
    }

    pub fn create(this: Uuid, cx: &mut Context, x: f32, y: f32) -> Node {
        let width =  145.0;
        let height = 34.0;
        let mut node = Node::new(this, x, y, width, height);
        node.set_opacity(1.0);
        node.set_texture(Some(Node::create_texture(cx, "/assets/button.png")));
        node
    }

    pub fn create_pressed(this: Uuid, cx: &mut Context, x: f32, y: f32) -> Node {
        let width =  145.0;
        let height = 34.0;
        let mut node = Node::new(this, x, y, width, height);
        node.set_opacity(0.0);
        node.set_texture(Some(Node::create_texture(cx, "/assets/button_pressed.png")));
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
            advance = advance + (word.get_advance_for_char(c as usize));   
            
            web_sys::console::log_1(&advance.to_string().into());         
            cx.nodes.push(node);
            cx.add_child_to(node_uuid_parent, node_uuid);
        }
        node.set_x((145.0 - advance) / 2.0);
        node.set_y(10.0);
        cx.nodes.push(node);
        node_uuid_parent
    }

    pub fn on_button_pressed(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set on".into());
        match self.state {
            ButtonState::NotPressed => 
            {
                web_sys::console::log_1(&"ButtonState::NotPressed".into());
                self.play_on_animation(cx)
            },
            ButtonState::Pressed => 
            {
                web_sys::console::log_1(&"ButtonState::Pressed".into());
                self.play_off_animation(cx)
            },
            ButtonState::ToNotPressed => 
            {
                web_sys::console::log_1(&"ButtonState::ToNotPressed".into());
                self.set_off(cx);
                self.play_on_animation(cx);
            },
            ButtonState::ToPressed => 
            {
                web_sys::console::log_1(&"ButtonState::ToPressed".into());
                self.set_on(cx);
                self.play_off_animation(cx);
            },
        }
    }

    pub fn on_animation_ended(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"on_animation_ended".into());
        match self.state {
            ButtonState::NotPressed => (),
            ButtonState::Pressed => (),
            ButtonState::ToNotPressed  => self.set_off(cx),
            ButtonState::ToPressed  => self.set_on(cx)
        }
    }

    pub fn set_on(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set Pressed".into());
        self.state = ButtonState::Pressed;
        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().start_value = 1.0;
        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().end_value = 0.0;
    }

    fn set_off(&mut self, cx: &mut Context) {
        web_sys::console::log_1(&"set NotPressed".into());
        self.state = ButtonState::NotPressed;
        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().start_value = 0.0;
        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().end_value = 1.0;
    }

    fn play_off_animation(&mut self, context: &mut Context) {
        web_sys::console::log_1(&"play ToNotPressed".into());
        self.state = ButtonState::ToNotPressed;
        let off_animation = context.get_animation(self.off_animation_uuid);
        off_animation.unwrap().play();
    }

    fn play_on_animation(&mut self, context: &mut Context) {
        web_sys::console::log_1(&"play ToPressed".into());
        self.state = ButtonState::ToPressed;
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

    fn event_handler(&mut self, cx: &mut Context, message: &Event, target: Uuid) -> bool {
    //        web_sys::console::log_1(&target.to_string().into());
    //        web_sys::console::log_1(&self.this.to_string().into());
    //        web_sys::console::log_1(&"got event".into());
            match message {
                Event::Mouse(event) => {
                    if target == self.this {
                        if event.event == MouseEvent::Up {
                            web_sys::console::log_1(&"mouse up".into());
                            if self.pressed == true {
                                self.pressed = false;
                                self.on_button_pressed(cx);
                                let _  = cx.messaging.send_message(MessageType::ValueUpdated(self.this, "onClicked".to_owned(), "true".to_owned()));
                            }
                        } else if event.event == MouseEvent::Down {
                            self.pressed = true; 
                            self.on_button_pressed(cx);
                        } else if event.event == MouseEvent::In {
                            web_sys::console::log_1(&"mouse in".into());
                        } else if event.event == MouseEvent::Out {
                            web_sys::console::log_1(&"mouse out".into());
                            if self.pressed == true {
                                web_sys::console::log_1(&"mouse cancel".into());
                                self.pressed = false;
                                self.play_off_animation(cx);
                            }
                        }
                    }
    //                return true;
                },
                Event::Message(message) => {
                    web_sys::console::log_1(&"message".into());
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
