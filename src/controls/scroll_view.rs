use crate::events::mouse::*;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::application::context::*;
use uuid::Uuid;
use std::any::Any;
use serde::*;

use uimsg::MessageType;

#[derive(PartialEq, Clone, Copy)]
pub struct ScrollViewPrivate {
    this: Uuid,
    node_uuid: Uuid,
    scroll_uuid: Uuid,
    scroll_x: Option<f32>,
    scroll_y: Option<f32>
}

impl ScrollViewPrivate {

    pub fn from_public(cx: &mut Context, public: ScrollView) -> ScrollViewPrivate {
        ScrollViewPrivate::new(public.this, cx, public.x, public.y, public.translate_x, public.translate_y, public.opacity, public.width, public.height, (public.color.r, public.color.g, public.color.b))
    }

    pub fn new(owner: Uuid, cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color:(f32,f32,f32)) ->  ScrollViewPrivate {
        
        let owner = owner;

        let root_node = ScrollViewPrivate::new_node(owner, cx, x, y, translate_x, translate_y, opacity, width, height, color, true);
        let node_uuid = root_node.uuid;

        let scroll_node = ScrollViewPrivate::new_node(owner, cx, 0.0, 0.0, 0.0, 0.0, opacity, width, height, (1.0,1.0,1.0), false);
        let scroll_node_uuid = scroll_node.uuid;

        cx.add_child_to(node_uuid, scroll_node_uuid);
        cx.nodes.push(root_node);
        cx.nodes.push(scroll_node);

        ScrollViewPrivate { this: owner, node_uuid, scroll_uuid: scroll_node_uuid, scroll_x: None, scroll_y: None}
    }

    pub fn setup_scroll(&self, cx: &mut Context, child: Uuid) {
        let content = cx.get_node_unmut(child);
        let width = content.unwrap().width;
        let height = content.unwrap().height;

        let mut scrolling = cx.get_node(self.scroll_uuid).unwrap();
        scrolling.width = width;
        scrolling.height = height;

        cx.add_child_to(self.scroll_uuid, child);
/*
        web_sys::console::log_1(&"x:".into());
        web_sys::console::log_1(&self.get_max_scroll_x(cx).to_string().into());
        web_sys::console::log_1(&"y:".into());
        web_sys::console::log_1(&self.get_max_scroll_y(cx).to_string().into());
*/        
    }
    
    fn start_scroll_event(&mut self, cx: &mut Context, message: &Mouse) {
        let node = cx.get_node(self.scroll_uuid).unwrap();
/*
        web_sys::console::log_1(&"x:".into());
        web_sys::console::log_1(&node.translate_x.to_string().into());
        web_sys::console::log_1(&"y:".into());
        web_sys::console::log_1(&node.translate_y.to_string().into());

        web_sys::console::log_1(&"x:".into());
        web_sys::console::log_1(&message.x.to_string().into());
        web_sys::console::log_1(&"y:".into());
        web_sys::console::log_1(&message.y.to_string().into());
*/
        self.scroll_x = Some(message.x as f32 - node.translate_x); //need to add previous scrolling
        self.scroll_y = Some(message.y as f32 - node.translate_y) ; 
    }

    fn on_scroll_event(&mut self, cx: &mut Context, message: &Mouse) {
        web_sys::console::debug_1(&"on scroll event".into());
        let max_scroll_x = self.get_max_scroll_x(cx); 
        let max_scroll_y = self.get_max_scroll_y(cx); 
        let min_scroll_x = self.get_min_scroll_x(cx); 
        let min_scroll_y = self.get_min_scroll_y(cx); 
        let mut node = cx.get_node(self.scroll_uuid).unwrap();
        
        if let Some(x) = self.scroll_x {
            let  wanted_scroll = message.x as f32 - x; 
            if wanted_scroll < max_scroll_x && wanted_scroll > min_scroll_x {
                node.translate_x = wanted_scroll;
            } else if wanted_scroll < min_scroll_x {
                node.translate_x = min_scroll_x;
            } else {
                node.translate_x = max_scroll_x;
            }
        }
        if let Some(y) = self.scroll_y {
            let  wanted_scroll = message.y as f32 - y; 
            if wanted_scroll < max_scroll_y && wanted_scroll > min_scroll_y { 
                node.translate_y = wanted_scroll;
            } else if wanted_scroll < min_scroll_y {
                node.translate_y = min_scroll_y;
            } else {
                node.translate_y = max_scroll_y;
            }
        }
        web_sys::console::debug_4(&"t_x".into(),&node.translate_x.into(),&"t_y".into(),&node.translate_y.into());
        {
            let mut dirty = cx.dirty.borrow_mut();
            *dirty = true;
        }
    }

    fn get_max_scroll_x(&self, _cx: &Context) -> f32 {
        0.0
    } 

    fn get_max_scroll_y(&self, _cx: &Context) -> f32 {
        0.0
    } 

    fn get_min_scroll_y(&self, cx: &Context) -> f32 {
        let self_node = cx.get_node_unmut(self.node_uuid);
        let scroll_node = cx.get_node_unmut(self.scroll_uuid);
        self_node.unwrap().height - scroll_node.unwrap().height
    } 

    fn get_min_scroll_x(&self, cx: &Context) -> f32 {
        let self_node = cx.get_node_unmut(self.node_uuid);
        let scroll_node = cx.get_node_unmut(self.scroll_uuid);
        self_node.unwrap().width - scroll_node.unwrap().width
    } 

    fn end_scroll_event(&mut self, _cx: &mut Context, _message: &Mouse) {
        self.scroll_x = None;
        self.scroll_y = None;
    }

    fn new_node(owner: Uuid, _cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color:(f32,f32,f32), clip: bool) ->  Node {
        let x:f32 = x;
        let y:f32 = y;
        let translate_x = translate_x;
        let translate_y = translate_y;
        let opacity:f32 = opacity;
        let width = width;
        let height = height;
        let texture = None;
        let color = color;
        let dirty = true;
        let end_clip = false;
        let uuid = Uuid::new_v4();
        let text = false;
        Node { owner, uuid, x, y, width, height, translate_x, translate_y, opacity, texture, color, dirty, clip, end_clip, text }
    }
}

impl VisualNode for ScrollViewPrivate {

    fn get_node_uuid(&self) -> Uuid {
        self.node_uuid
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
                    web_sys::console::debug_1(&"Scrollview: Mouse up!".into());
                } else if event.event == MouseEvent::Down {
                    self.start_scroll_event(cx, &event);
                    web_sys::console::debug_1(&"crollview: Mouse down!".into());
                } else if event.event == MouseEvent::Move {
                    self.on_scroll_event(cx, &event);
                    web_sys::console::debug_1(&"crollview: Mouse move!".into());
                } 
//                return true;
            },
            Event::Message(_message) => {
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


#[derive(PartialEq, Clone, Copy, Deserialize)]
pub struct ScrollView {
    pub this: Uuid,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub opacity: f32,
    pub color: Color,
    pub clip: bool,
}

#[derive(PartialEq, Clone, Copy, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
} 
