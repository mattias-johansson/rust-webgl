use serde::*;
use uuid::Uuid;


#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MessageType {
    ObjectCreated(Uuid, String, String),
    ValueUpdated(Uuid, String, String),
    SetupScrollView(Uuid, String),
    AddNode(String),
    RemoveNode(String),
    SetRoot(String)
}