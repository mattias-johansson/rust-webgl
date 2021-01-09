use crate::events::mouse::*;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::application::context::*;
use uuid::Uuid;
use std::any::Any;
use crate::animation::animation::*;
use crate::animation::ease::*;

#[derive(PartialEq, Clone, Copy)]
pub struct Slider {
    pub this: Uuid,
    slider_node_active: Uuid,
    slider_node_inactive: Uuid,
    background_node: Uuid,
    start_x: Option<f32>,
    on_animation_uuid: Uuid,
    off_animation_uuid: Uuid,
}

impl Slider {
    pub fn new(cx: &mut Context, x: f32, y: f32, opacity: f32) -> Slider {
        let this = Uuid::new_v4();
        let background_node = Slider::background(cx, this, x, y);
        let handle_pressed_node = Slider::handle_pressed(cx, this, 0.0, 0.0);
        let handle_node = Slider::handle_inactive(cx, this, 0.0, 0.0);

        let background_node_uuid = background_node.uuid;
        let handle_pressed_node_uuid  = handle_pressed_node.uuid;
        let handle_node_uuid = handle_node.uuid;

        let mut on_animation = Animation::new(handle_pressed_node_uuid, Attribute::OPACITY);
        on_animation.duration = 100.0;
        on_animation.start_value = 0.0;
        on_animation.end_value = 1.0;
        on_animation.easing = Ease::InCubic;

        let mut off_animation = Animation::new(handle_pressed_node_uuid, Attribute::OPACITY);
        off_animation.duration = 100.0;
        off_animation.start_value = 1.0;
        off_animation.end_value = 0.0;
        off_animation.easing = Ease::InCubic;

        let on_animation_uuid = on_animation.uuid;
        let off_animation_uuid = off_animation.uuid;

        cx.nodes.push(background_node);
        cx.nodes.push(handle_node);
        cx.nodes.push(handle_pressed_node);

        cx.animations.push(on_animation);
        cx.animations.push(off_animation);

        cx.add_child_to(background_node_uuid, handle_node_uuid);
        cx.add_child_to(background_node_uuid, handle_pressed_node_uuid);

        Slider { this: this, slider_node_active: handle_pressed_node_uuid, slider_node_inactive: handle_node_uuid, background_node: background_node_uuid, start_x : None, on_animation_uuid, off_animation_uuid }
    }

    fn background(cx: &mut Context, this: Uuid, x: f32, y: f32) -> Node {
        let mut node = Node::new(this, x, y, 447.0, 60.0);
        node.opacity = 1.0;
        node.texture = Some(Node::create_texture(cx, "/assets/slider_track.png"));
        node
    }

    fn handle_pressed(cx: &mut Context, this: Uuid, x: f32, y: f32) -> Node {
        let mut node = Node::new(this, x, y, 60.0, 60.0);
        node.opacity = 0.0;
        node.texture = Some(Node::create_texture(cx, "/assets/handle_pressed.png"));
        node
    }
    
    fn handle_inactive(cx: &mut Context, this: Uuid, x: f32, y: f32) -> Node {
        let mut node = Node::new(this, 14.0, 14.0, 30.0, 30.0);
        node.opacity = 1.0;
        node.texture = Some(Node::create_texture(cx, "/assets/handle_inactive.png"));
        node
    }

    fn start_scroll_event(&mut self, cx: &mut Context, message: &Mouse) {

        let node = cx.get_node(self.slider_node_active).unwrap();
        self.start_x = Some(message.x as f32 - node.translate_x);
        let on_animation = cx.get_animation(self.on_animation_uuid);
        on_animation.unwrap().play();
    }

    fn on_scroll_event(&mut self, cx: &mut Context, message: &Mouse) {
        if self.start_x.is_some() {
            let position = message.x as f32 - self.start_x.unwrap();
            let node_bg = cx.get_node_unmut(self.background_node).unwrap();
            let max_slider = node_bg.width;
            let node = cx.get_node(self.slider_node_active).unwrap();
            node.translate_x = position;
            let node = cx.get_node(self.slider_node_inactive).unwrap();
            node.translate_x = position;
            let percent = position / max_slider;
            cx.cb.add_trigged(self.this, Event::Scroll(Scroll::ImmediateValue(percent)));
        }
    }

    fn end_scroll_event(&mut self, cx: &mut Context, message: &Mouse) {
        self.start_x = None;
        let off_animation = cx.get_animation(self.off_animation_uuid);
        off_animation.unwrap().play();
    }
    
}

impl VisualNode for Slider {

    fn get_node_uuid(&self) -> Uuid {
        self.background_node
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool {
//        web_sys::console::log_1(&"got event".into());
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                    self.end_scroll_event(cx, &event);
                    web_sys::console::log_1(&"SLIDER: Mouse up!".into());
                } else if event.event == MouseEvent::Down {
                    self.start_scroll_event(cx, &event);
                } else if event.event == MouseEvent::Move {
                    self.on_scroll_event(cx, &event);
                } 
                return true;
            },
            Event::Message(message) => {
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