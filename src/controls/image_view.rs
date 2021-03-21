use std::any::Any;
use uuid::Uuid;
use crate::application::context::*;
use crate::controls::visual_node::VisualNode;
use crate::controls::node::*;
use crate::events::mouse::*;
use serde::*;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ImageViewPrivate {
    this: Uuid,
    node: Uuid
}

impl VisualNode for ImageViewPrivate {

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

impl ImageViewPrivate {

    pub fn from_public(cx: &mut Context, public: ImageView) -> ImageViewPrivate {
        ImageViewPrivate::new(public.this, cx, public.x, public.y, public.translate_x, public.translate_y, public.opacity, public.width, public.height, public.image.as_str())
    }

    pub fn new(this: Uuid, cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, texture: &str) -> ImageViewPrivate {
        let x:f32 = x;
        let y:f32 = y;
        let translate_x = translate_x;
        let translate_y = translate_y;
        let opacity:f32 = opacity;
        let width = width;
        let height = height;
        let texture = Some(Node::create_texture(cx, texture));
        let color = (0.0, 0.0, 0.0);
        let dirty = true;
        let clip = false;
        let end_clip = false;
        let uuid = Uuid::new_v4();
        let owner = this;
        let text = false;
        let node = Node::create(owner, uuid, x, y, width, height, translate_x, translate_y, opacity, texture, color, dirty, clip, end_clip, text);
        let node_uuid = node.uuid;
        cx.nodes.push(node);
        ImageViewPrivate { this: owner, node: node_uuid }
    }
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct ImageView {
    pub this: Uuid,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub opacity: f32,
    pub image: String,
}