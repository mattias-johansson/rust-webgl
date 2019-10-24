use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
}
