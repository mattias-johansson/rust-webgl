
use crate::render::word::*;

use uuid::Uuid;
use crate::application::context::*;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use std::any::Any;

use crate::events::mouse::*;

#[derive(PartialEq, Clone)]
pub struct Label {
    this: Uuid,
    pub node: Uuid,
    pub word: Word,
    pub text: String,

}

impl Label {

    pub fn new(cx: &mut Context, text: &str) -> Label {
        let this = Uuid::new_v4();
        let mut node = Node::new(this, 0.0, 0.0, 100.0, 100.0);
        node.text = true;
        let node_uuid = node.uuid;
        let text = String::from(text);
        let mut word = Word::default();
        word.create_char_points_for_text(&text);
        let mut char_iter = text.chars();
        while let Some(c) = char_iter.next() {
            cx.vertices.insert(node_uuid, word.get_char_points_for_char(&(c as usize)).unwrap().to_vec());
        }
        cx.nodes.push(node);
        Label { this, node: node_uuid, word, text }

    }

}

impl VisualNode for Label {
    fn get_node_uuid(&self) -> Uuid {
        self.node
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool{
        return false;
    }

    fn get_uuid(&self) -> uuid::Uuid { 
        self.this
    }
}