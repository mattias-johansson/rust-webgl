
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

    pub fn new(cx: &mut Context, x: f32, y: f32, text: &str) -> Label {
        let this = Uuid::new_v4();
        let node = Node::new(this, x, y, 0.0, 0.0);
        let node_uuid_parent = node.uuid;
        cx.nodes.push(node);
        let text = String::from(text);
        let mut word = Word::default();
        word.create_char_points_for_text(&text);
        let mut char_iter = text.chars();
        let mut advance: f32 = 0.0;
        while let Some(c) = char_iter.next() {
            let mut node = Node::new(this, x + advance, y, 100.0, 100.0);
            node.text = true;
            let node_uuid = node.uuid;
            cx.vertices.insert(node.uuid, word.get_char_points_for_char(&(c as usize)).unwrap().to_vec());
            advance = advance + (word.get_advance_for_char(c as usize) * 0.009);   
            
            web_sys::console::log_1(&advance.to_string().into());         
            cx.nodes.push(node);
            cx.add_child_to(node_uuid_parent, node_uuid);

        }
        Label { this, node: node_uuid_parent, word, text }

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