
#[derive(Clone)]
pub struct ScrollView {
    this: Uuid,
    node: node_uuid
}

impl VisualNode for ScrollView {

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