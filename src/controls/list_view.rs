use core::any::Any;
use crate::CoreApp;
use crate::LabelPrivate;
use crate::events::mouse::*;
use crate::controls::visual_node::*;
use crate::controls::node::*;
use crate::application::context::*;
use uuid::Uuid;
use serde::*;


#[derive(Clone)]
pub struct ListViewPrivate {
    pub this: Uuid,
    node_uuid: Uuid,
    scroll_uuid: Uuid,
    scroll_x: Option<f32>,
    scroll_y: Option<f32>,
}

pub struct ListTraitHolder {
    pub item_provider: Box<dyn ListItemProvider>,
    pub data_model: Box<dyn ListDataModel>,
}

pub trait ListItemProvider {
    fn create_item(self, cx: &mut Context, core_app: &mut CoreApp, x: f32, y: f32, width: f32, height: f32, text: &str) -> Uuid;
    fn update_item(self, list_item: &mut LabelPrivate, cx: &mut Context, text: &str);
}

pub trait ListDataModel {
//    itemAdded(&mut self, cx: &mut Context)
//    itemMoved(&mut self, cx: &mut Context)
//    itemRemoved(&mut self, cx: &mut Context)
//    itemUpdated(&mut self, cx: &mut Context)
    fn item_count(&self) -> usize;
    fn data(&self, index: usize) -> &str;

}

impl ListViewPrivate {
    pub fn from_public(cx: &mut Context, public: ListView) -> ListViewPrivate {
        ListViewPrivate::new(public.this, cx, public.x, public.y, public.translate_x, public.translate_y, public.opacity, public.width, public.height, (1.0, 1.0, 1.0))
    }

    pub fn new(owner: Uuid, cx: &mut Context, x:f32, y:f32, translate_x:f32, translate_y:f32, opacity:f32, width:f32, height:f32, color:(f32,f32,f32)) ->  ListViewPrivate {
        
        let owner = owner;

        let root_node = ListViewPrivate::new_node(owner, cx, 0.0, 0.0, 0.0, 0.0, 1.0, 300.0, 300.0, color, true);
        let node_uuid = root_node.uuid;

        let scroll_node = ListViewPrivate::new_node(owner, cx, 0.0, 0.0, 0.0, 0.0, 1.0, 100.0, 100.0, color, false);
        let scroll_node_uuid = scroll_node.uuid;

        cx.add_child_to(node_uuid, scroll_node_uuid);
        cx.nodes.push(root_node);
        cx.nodes.push(scroll_node);

        web_sys::console::debug_1(&"new scroll event".into());
        ListViewPrivate { this: owner, node_uuid, scroll_uuid: scroll_node_uuid, scroll_x: None, scroll_y: None}
    }

    pub fn setup_list(&self, cx: &mut Context) {

        web_sys::console::debug_1(&"setup_scroll 1".into());
        let item1 = LabelPrivate::new(Uuid::new_v4(), cx, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,  &"item1");
        cx.add_child_to(self.scroll_uuid, item1.get_node_uuid());
 
        let item2 = LabelPrivate::new(Uuid::new_v4(), cx, 0.0, 20.0, 0.0, 0.0, 0.0, 0.0, 0.0,  &"item2");
        cx.add_child_to(self.scroll_uuid, item2.get_node_uuid());
 
        let item3 = LabelPrivate::new(Uuid::new_v4(), cx, 0.0, 40.0, 0.0, 0.0, 0.0, 0.0, 0.0,  &"item3");
        cx.add_child_to(self.scroll_uuid, item3.get_node_uuid());
        
        let scrolling = cx.get_node(self.scroll_uuid).unwrap();
//        scrolling.set_width(100.0);
        scrolling.set_height(300.0);

        web_sys::console::debug_1(&"setup_scroll 2".into());
    }

    fn start_scroll_event(&mut self, cx: &mut Context, message: &Mouse) {
        let node = cx.get_node(self.scroll_uuid).unwrap();
        self.scroll_x = Some(message.x as f32 - node.translate_x()); //need to add previous scrolling
        self.scroll_y = Some(message.y as f32 - node.translate_y()) ; 
    }

    fn on_scroll_event(&mut self, cx: &mut Context, message: &Mouse) {
        web_sys::console::debug_1(&"on scroll event".into());
        let max_scroll_x = self.get_max_scroll_x(cx); 
        let max_scroll_y = self.get_max_scroll_y(cx); 
        let min_scroll_x = self.get_min_scroll_x(cx); 
        let min_scroll_y = self.get_min_scroll_y(cx); 
        let node = cx.get_node(self.scroll_uuid).unwrap();
        
        if let Some(x) = self.scroll_x {
            let  wanted_scroll = message.x as f32 - x; 
            
            if wanted_scroll < max_scroll_x && wanted_scroll > min_scroll_x {
                node.set_translate_x(wanted_scroll);
            } else if wanted_scroll < min_scroll_x {
                node.set_translate_x(min_scroll_x);
            } else {
                node.set_translate_x(max_scroll_x);
            }
        }
        if let Some(y) = self.scroll_y {
            let  wanted_scroll = message.y as f32 - y; 
            if wanted_scroll < max_scroll_y && wanted_scroll > min_scroll_y { 
                node.set_translate_y(wanted_scroll);
            } else if wanted_scroll < min_scroll_y {
                node.set_translate_y(min_scroll_y);
            } else {
                node.set_translate_y(max_scroll_y);
            }
        }
    //        web_sys::console::debug_4(&"t_x".into(),&node.translate_x.into(),&"t_y".into(),&node.translate_y.into());
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
        self_node.unwrap().height() - scroll_node.unwrap().height()
    } 

    fn get_min_scroll_x(&self, cx: &Context) -> f32 {
        let self_node = cx.get_node_unmut(self.node_uuid);
        let scroll_node = cx.get_node_unmut(self.scroll_uuid);
        self_node.unwrap().width() - scroll_node.unwrap().width()
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
        Node::create(owner, uuid, x, y, width, height, translate_x, translate_y, opacity, texture, color, dirty, clip, end_clip, text)
    }
}


impl VisualNode for ListViewPrivate {

    fn get_node_uuid(&self) -> Uuid {
        self.node_uuid
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event_handler(&mut self, cx: &mut Context, message: &Event, _target: Uuid) -> bool {
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

#[derive(Serialize, Deserialize)]
pub struct ListView {
    pub this: Uuid,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub opacity: f32,
}

