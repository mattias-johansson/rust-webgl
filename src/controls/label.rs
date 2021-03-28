
use crate::render::word::*;

use uuid::Uuid;
use crate::application::context::*;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use std::any::Any;

use crate::events::mouse::*;
use serde::*;

#[derive(PartialEq, Clone)]
pub struct LabelPrivate {
    this: Uuid,
    pub node: Uuid,
    pub word: Word,
    pub text: String,
}

impl LabelPrivate {

    pub fn from_public(cx: &mut Context, public: Label) -> LabelPrivate {
        LabelPrivate::new(public.this, cx, public.x, public.y, public.translate_x, public.translate_y, public.opacity, public.width, public.height, public.text.as_str())
    }

    pub fn new(this: Uuid, cx: &mut Context, x: f32, y: f32, _translate_x:f32, _translate_y:f32, _opacity:f32, width:f32, height:f32, text: &str) -> LabelPrivate {
        let node = Node::new(this, x, y, width, height);
        let node_uuid_parent = node.uuid;
        cx.nodes.push(node);
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
//            web_sys::console::log_1(&advance.to_string().into());         
            cx.nodes.push(node);
            cx.add_child_to(node_uuid_parent, node_uuid);

        }
        LabelPrivate { this, node: node_uuid_parent, word, text }
    }

    pub fn text(&mut self, cx: &mut Context, text: String) {
        let parent_y;
        let parent_uuid;
        {
            parent_uuid = cx.get_node_unmut(self.node).unwrap().uuid;
            parent_y = cx.get_node_unmut(self.node).unwrap().y();
        }
        cx.remove_all_child_from(parent_uuid);
        self.word.create_char_points_for_text(&text);
        let mut char_iter = text.chars();
        let mut advance: f32 = 0.0;
        while let Some(c) = char_iter.next() {
            let mut node = Node::new(self.this, advance, parent_y, 100.0, 100.0);
            node.text = true;
            let node_uuid = node.uuid;
            cx.vertices.insert(node.uuid, self.word.get_char_points_for_char(&(c as usize)).unwrap().to_vec());
            advance = advance + (self.word.get_advance_for_char(c as usize) * 0.009);   
//            web_sys::console::log_1(&advance.to_string().into());         
            cx.nodes.push(node);
            cx.add_child_to(parent_uuid, node_uuid);

        }
        let mut dirty = cx.dirty.borrow_mut();
        *dirty = true;
    }

}

impl VisualNode for LabelPrivate {

    fn get_node_uuid(&self) -> Uuid {
        self.node
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, _cx: &mut Context, _message: &Event) -> bool{
        return false;
    }

    fn get_uuid(&self) -> uuid::Uuid { 
        self.this
    }
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct Label {
    pub this: Uuid,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub opacity: f32,
    pub text: String,
}