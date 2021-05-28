
use crate::controls::list_view::ListItemProvider;
use crate::CoreApp;
use crate::LabelPrivate;
use crate::controls::visual_node::*;
use crate::application::context::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct SimpleListItemProvider {

}

impl ListItemProvider for SimpleListItemProvider {

    fn create_item(self, cx: &mut Context, core_app: &mut CoreApp, x: f32, y: f32, width: f32, height: f32, text: &str) -> Uuid {
        let this = Uuid::new_v4();
        let label = LabelPrivate::new(this, cx, x, y, 0.0, 0.0, 0.0, width, height, text);
        let uuid = label.get_uuid();
        core_app.visual_nodes.push(Box::new(label) as Box<dyn VisualNode>);
        uuid
    }

    fn update_item(self, list_item: &mut LabelPrivate, cx: &mut Context, text: &str) where Self : Sized {
        list_item.text(cx, text);
    }
}