
use crate::events::mouse::*;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::application::context::*;
use uuid::Uuid;
use std::any::Any;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ScrollView {
    this: Uuid,
    node_uuid: Uuid,
    scroll_uuid: Uuid
}

impl ScrollView {

    pub fn new(cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color:(f32,f32,f32)) ->  ScrollView {
        
        let owner = Uuid::new_v4();

        let root_node = ScrollView::new_node(owner, cx, x, y, translate_x, translate_y, opacity, width, height, color, true);
        let node_uuid = root_node.uuid;

        let scroll_node = ScrollView::new_node(owner, cx, 0.0, 0.0, 0.0, 0.0, opacity, width, height, (1.0,1.0,1.0), false);
        let scroll_node_uuid = scroll_node.uuid;

        cx.add_child_to(node_uuid, scroll_node_uuid);
        cx.nodes.push(root_node);
        cx.nodes.push(scroll_node);


        ScrollView { this: owner, node_uuid, scroll_uuid: scroll_node_uuid}
    }

    pub fn add_content(&self, cx: &mut Context, child:Uuid) {
        cx.add_child_to(self.scroll_uuid, child);
    }

    fn on_scroll_event(&mut self, cx: &mut Context, message: &MouseEvent) {
        let mut node = cx.get_node(self.scroll_uuid).unwrap();
        node.translate_x = node.translate_x + 5.0; 
    }

    fn new_node(owner: Uuid, cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color:(f32,f32,f32), clip: bool) ->  Node {
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

impl VisualNode for ScrollView {

    fn get_node_uuid(&self) -> Uuid {
        self.node_uuid
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event) -> bool {
        web_sys::console::log_1(&"got event".into());
        match message {
            Event::Mouse(event) => {
                if event.event == MouseEvent::Up {
                } else if event.event == MouseEvent::Down {
                } else if event.event == MouseEvent::Move {
                    self.on_scroll_event(cx, &event.event);
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