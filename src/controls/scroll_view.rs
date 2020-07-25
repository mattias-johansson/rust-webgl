
#[derive(Clone)]
pub struct ScrollView {
    this: Uuid,
    pub node: Node
}

impl VisualNode for ScrollView {

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