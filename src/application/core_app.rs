
use crate::controls::visual_node::VisualNode;

pub struct CoreApp {
    pub visual_nodes: Vec<Box <dyn VisualNode>>
}